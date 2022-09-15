use leafwing_input_manager::Actionlike;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum InputAction {
    Shoot,
    Move_Up,
    Move_Down,
    Move_Left,
    Move_Right,
}
