@tool
extends RefCounted

const ScreenshotScriptContext := preload(
	"res://addons/fennara/runtime/screenshot_script_context.gd"
)

signal completed


func execute(instance: RefCounted, ctx: RefCounted) -> void:
	var public_ctx := ScreenshotScriptContext.new(ctx)
	await instance.call("run", public_ctx)
	_emit_completed.call_deferred()


func _emit_completed() -> void:
	completed.emit()
