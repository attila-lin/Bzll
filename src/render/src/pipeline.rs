
use fnv::FnvHashMap as HashMap;

/// A stackable image layer.
///
/// Layers contain a list of passes which are used to render a final image which
/// is then composited onto a render target. They are especially useful for
/// postprocessing, e.g. applying a fullscreen night vision effect, drawing a
/// HUD (heads-up display) over a rendered scene.
pub struct Layer {
    /// Name of the render target to draw on.
    pub target: String,
    /// Sequence of passes to execute over the render target.
    pub passes: Vec<Box<PassDescription>>,
}

impl Layer {
    /// Creates a new layer with the given list of passes and the name of the
    /// render target
    pub fn new<T>(target: T, passes: Vec<Box<PassDescription>>) -> Layer
        where String: From<T>
    {
        Layer {
            target: String::from(target),
            passes: passes,
        }
    }
}

/// The render job submission
/// Describes the layers and
pub struct Pipeline {
    /// the layers to be processed
    pub layers: Vec<Layer>,
    /// collection of render targets. A target may be
    /// a source or a sink for a `Pass`
    pub targets: HashMap<String, Box<Target>>,
}

impl Pipeline {
    /// Create an empty Pipeline
    pub fn new() -> Pipeline {
        Pipeline {
            layers: Vec::new(),
            targets: HashMap::default(),
        }
    }
}
