use super::connected_shutdown_error;

#[test]
fn shutdown_is_allowed_without_connected_godot_projects() {
    assert!(connected_shutdown_error(0).is_none());
}

#[test]
fn shutdown_reports_connected_project_count() {
    let error = connected_shutdown_error(2).unwrap();
    assert_eq!(error["error"], "connected_godot_projects");
    assert_eq!(error["connected_project_count"], 2);
}
