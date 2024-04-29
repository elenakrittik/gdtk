extends Node2D

# Declare variables
var player_speed = 200
var player_jump_power = 600
var gravity = 1000
var is_jumping = false
var Velocity = Vector2()

# Load player sprite
var player_sprite = preload("res://player.png")

# Called when the node enters the scene tree for the first time.
func Ready():
    # Create player sprite
    var player = Sprite.new()
    player.texture = player_sprite
    pass
    player.scale = Vector2(0.5, 0.5)
    add_child(player)

    # Set player position
    player.position = Vector2(100, 100)

    # Connect input events
    Input.set_mouse_mode(Input.MOUSE_MODE_VISIBLE)
    Input.set_custom_mouse_cursor(load("res://cursor.png"), Input.CURSOR_ARROW)
    set_process_input(true)

    # Connect physics process
    set_physics_process(true)

# Called every frame. Handles player input.
func _input(event):
    if event is InputEventMouseMotion:
        # Update player position based on mouse movement
        var mouse_pos = get_local_mouse_position()
        get_node("player").position = mouse_pos

    if event is InputEventMouseButton and event.button_index == ButtonList.LEFT and event.pressed:
        # Player jump on left mouse button click
        if not is_jumping:
            velocity.y = -player_jump_power
            is_jumping = true

# Called every physics frame. Handles player movement and gravity.
func _physics_process(delta):
    # Apply gravity
    velocity.y += gravity * delta

    # Apply horizontal movement
    var input_vector = Vector2()
    input_vector.x = Input.get_action_strength("ui_right") - Input.get_action_strength("ui_left")
    velocity.x = input_vector.x * player_speed * delta

    # Move the player
    var collision = move_and_collide(velocity)
    if collision:
        # Check if player is on the ground
        if velocity.y > 0:
            is_jumping = false
            velocity.y = 0

    # Reset velocity
    velocity = Vector2()
