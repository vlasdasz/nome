// The lib gates its `register_app!` behind `cfg(ios)`, and the weak
// `hilen_create_app` stub in an rlib would not be overridden from
// there anyway. The final crate must register, like the desktop binary.
hilen::register_app!(nome::NomeApp);

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: hilen::AndroidApp) {
    hilen::hilen_start_app(app);
}
