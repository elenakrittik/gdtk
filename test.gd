# Test variable declaration
var num = 12

# Test typed variable declation
var num: int = 12

# Test minimal annotation
@onready

# Test parentised annotation
@onready()

# Test annotation with one arg
@onready("arg")

# Test annotation with several args
@onready("arg1", "arg2")

# Test annotation with one arg and trailing comma
@onready("arg",)

# Test annotation with several args and trailing comma
@onready("arg1", "arg2",)

# Test minimal annotation with text after it
@onready var number = 1

# Test parentised annotation with text after it
@onready() var number = 1
