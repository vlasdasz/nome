// The lib gates its `register_app!` behind `cfg(ios)`, and the weak
// `test_engine_create_app` stub in an rlib would not be overridden from
// there anyway. The final crate must register, like the desktop binary.
test_engine::register_app!(nome::NomeApp);

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: test_engine::AndroidApp) {
    test_engine::test_engine_start_app(app);
}
