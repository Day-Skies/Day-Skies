//! App settings: temperature unit and the Open-Meteo host, persisted via day-part-prefs and
//! exposed as signals the UI reads reactively. The page itself is a native Form (docs/forms.md):
//! a segmented unit picker and a host field, in grouped section cards on every platform.

use crate::res;
use day::prelude::*;
use std::cell::OnceCell;

/// The default forecast host; the settings page lets users point at an Open-Meteo-compatible
/// proxy instead.
pub const DEFAULT_HOST: &str = "api.open-meteo.com";

const PREF_UNIT: &str = "dayskies.unit"; // "c" | "f"
const PREF_HOST: &str = "dayskies.host";

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Unit {
    Celsius,
    Fahrenheit,
}

/// The settings signals, created once in the root scope (first access must happen during the
/// root build) and seeded from the persistent store.
struct Store {
    /// Picker index: 0 = Celsius, 1 = Fahrenheit.
    unit: Signal<usize>,
    /// Host name as edited in the field (persisted + applied by the Save action).
    host: Signal<String>,
}

thread_local! {
    static STORE: OnceCell<Store> = const { OnceCell::new() };
}

fn with_store<R>(f: impl FnOnce(&Store) -> R) -> R {
    STORE.with(|cell| {
        f(cell.get_or_init(|| {
            // App-lifetime signals: a detached scope (the matrix-core pattern), NOT the build
            // scope of whichever page happens to touch settings first — that scope is disposed
            // when the page rebuilds, and a disposed signal panics on read.
            Scope::detached().enter(|| {
                let unit = Signal::new(match day_part_prefs::get(PREF_UNIT).as_deref() {
                    Some("f") => 1,
                    _ => 0,
                });
                let host = Signal::new(
                    day_part_prefs::get(PREF_HOST).unwrap_or_else(|| DEFAULT_HOST.to_string()),
                );
                // The unit applies (and persists) immediately on selection; the reactive
                // temperature labels re-render from the signal, no refetch needed (the model
                // stays °C).
                watch(
                    move || unit.get(),
                    |new, _| {
                        day_part_prefs::set(PREF_UNIT, if *new == 1 { "f" } else { "c" });
                    },
                );
                Store { unit, host }
            })
        }))
    })
}

/// The selected unit (tracked read — reactive closures re-run when it changes).
pub fn unit() -> Unit {
    if with_store(|s| s.unit.get()) == 1 {
        Unit::Fahrenheit
    } else {
        Unit::Celsius
    }
}

/// A Celsius temperature formatted in the selected unit, e.g. "18°" / "64°" (tracked read).
pub fn temp(celsius: f64) -> String {
    format!("{}\u{00B0}", temp_value(celsius))
}

/// A Celsius temperature rounded in the selected unit (tracked read) — for Fluent args.
pub fn temp_value(celsius: f64) -> i64 {
    let v = match unit() {
        Unit::Celsius => celsius,
        Unit::Fahrenheit => celsius * 9.0 / 5.0 + 32.0,
    };
    v.round() as i64
}

/// The currently-applied forecast host (untracked — captured before background fetches).
pub fn current_host() -> String {
    let h = with_store(|s| s.host.get_untracked());
    let h = h.trim();
    if h.is_empty() {
        DEFAULT_HOST.to_string()
    } else {
        h.to_string()
    }
}

/// The settings page: a Form with a Units section (segmented picker) and a Server section
/// (host field + Save, which persists and refetches every city).
pub fn settings_page() -> AnyPiece {
    let (unit_sig, host_sig) = with_store(|s| (s.unit, s.host));

    // The segmented picker has no ArkUI backend yet; HarmonyOS gets a native toggle instead.
    #[cfg(not(target_env = "ohos"))]
    let unit_row = labeled(
        res::str::settings_unit_label(),
        picker(
            [
                res::str::unit_celsius().format(),
                res::str::unit_fahrenheit().format(),
            ],
            unit_sig,
        )
        .segmented()
        .id("unit-picker"),
    );
    #[cfg(target_env = "ohos")]
    let unit_row = {
        let fahrenheit = Signal::new(unit_sig.get_untracked() == 1);
        watch(
            move || fahrenheit.get(),
            move |on, _| unit_sig.set(if *on { 1 } else { 0 }),
        );
        labeled(
            res::str::unit_fahrenheit(),
            toggle(fahrenheit).id("unit-picker"),
        )
    };

    let save = button(res::str::settings_save())
        .action(move || {
            let host = host_sig.get_untracked();
            let host = host.trim();
            let host = if host.is_empty() { DEFAULT_HOST } else { host };
            host_sig.set(host.to_string());
            day_part_prefs::set(PREF_HOST, host);
            crate::reload_all();
        })
        .prominent()
        .id("settings-save");

    scroll(
        column((form((
            section((unit_row,)).title(res::str::settings_units_section()),
            section((
                labeled(
                    res::str::settings_host_label(),
                    text_field(host_sig)
                        .placeholder(DEFAULT_HOST.to_string())
                        .id("host-field"),
                ),
                label(res::str::settings_host_hint()).font(Font::Footnote),
                save,
            ))
            .title(res::str::settings_server_section()),
        )),))
        .align(HAlign::Leading)
        .padding(16.0),
    )
    .grow()
    .any()
}
