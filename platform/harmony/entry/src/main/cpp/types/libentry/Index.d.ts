// ArkTS type declaration for the Rust native module (libentry.so), registered by the C++ shim's
// NAPI init. `start` mounts the Day tree; the file-picker pair bridges Day's native open/save
// requests to the ArkTS @kit.CoreFileKit DocumentViewPicker (docs/files.md).
export const start: (content: Object, widthVp: number, heightVp: number, density: number) => void;

// Set a process environment variable BEFORE `start()`. The launcher (`day launch` → hdc
// `aa start --ps`) hands the app its dayscript engine port + token (and locale / autodrive) this
// way, and the EntryAbility applies them so the walkthrough runner can drive the running app —
// the HarmonyOS analogue of Android's intent-extra env delivery.
export const setEnv: (key: string, value: string) => void;

// Register the ArkTS file picker + the app cache dir. The callback is invoked (on the JS thread)
// when Day requests an open (mode 0) or save (mode 1); it must answer via `onFileResult`.
export const registerFilePicker: (
  callback: (req: number, mode: number, name: string, src: string, filters: string) => void,
  cacheDir: string
) => void;

// Report a picker result back to Day: the chosen local path, or "" if the user cancelled.
export const onFileResult: (req: number, path: string) => void;

// Hand the native side the app's ResourceManager so Day can read staged rawfile data resources
// (§18.3) via OH_ResourceManager_*. Call once, before or after `start()`; until then the rawfile
// resource opener returns nothing (day_ark_res_available == 0).
export const registerResourceManager: (resourceManager: Object) => void;

// Secondary day windows (docs/windows.md). The registered `open` launches a multiton
// DayWindowAbility (the day node id + title as want parameters); `close` terminates one.
// The ability page completes an open with `windowStart` (false = closed before connecting)
// and reports lifecycle through `windowResized` / `windowFocused` / `windowClosed`.
export const registerWindows: (
  open: (node: number, title: string) => void,
  close: (node: number) => void
) => void;
export const windowStart: (content: Object, node: number, widthVp: number, heightVp: number) => boolean;
export const windowResized: (node: number, widthVp: number, heightVp: number) => void;
export const windowFocused: (node: number, active: number) => void;
export const windowClosed: (node: number) => void;
