#[derive(Debug)]
/// Defines the actions that L-system symbols can correspond to.
pub enum Action {
    /// Draw forward by a certain distance.
    Forward(f64),
    /// Draw backwards by a certain distance.
    Backwards(f64),
    /// Rotate the drawing head by a certain angle, in radians. A positive angle means counterclockwise rotation.
    Rotate(f64),
    /// Push the state of the drawing head on the stack and use a new drawing head.
    StackPush,
    /// Pop a drawing head state from the stack and resume drawing with this state.
    StackPop
}