use std::cell::RefCell;
use std::f64::consts::PI;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::time::SystemTime;
use num_traits::Pow;
use rand::random;
use uuid::{Timestamp, Uuid};
use crate::system::TIME_INC;

/// Contains a list of synaptic variables that can be modulated by a modulatory synapse.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SynapticModVar {
    G,
    X,
    P,
    TX,
    W
}

/// Contains a list of neurite variables that can be modulated by a modulatory synapse.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NeuriteModVar {
    A,
    B,
    C,
    D,
    GCC,
    GPC,
    U,
    V,
    VR,
    VT,
    CAP
}

/// Contains the synaptic ID value for a synapse.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SynapticID {
    /// The synapse's ID value.
    id: Uuid,
}

// SynapticID function
impl SynapticID {
    /// Creates a new unique synaptic ID value.
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v7(Timestamp::now(SystemTime::now()))
        }
    }
}

/// Contains a list of synapse types.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SynapseType {
    Excitatory,
    Inhibitory,
    ShuntingInhibitory,
    SynapticModulator(SynapticModVar),
    NeuralModulator(NeuriteModVar),
    Gap
}

/// Contains a list of presynaptic input types.
#[derive(Clone, Debug, PartialEq)]
pub enum PresynapticInput {
    Electrode(Rc<RefCell<dyn Electrode>>),
    Sensor(Rc<RefCell<dyn Sensor>>),
    Neurite(Rc<RefCell<Neurite>>)
}

/// Trait for any type of electrode.
pub trait Electrode {
    /// Turns on (activates) the electrode.
    fn on(&mut self);

    /// Turns off (deactivates) the electrode.
    fn off(&mut self);

    /// Returns the voltage input of the electrode.
    fn voltage(&self) -> f64;

    /// Returns the duration of the electrode's pulse.
    fn duration(&self) -> f64;

    /// Returns the current output of the electrode.
    fn output(&self) -> f64;

    /// Process and returns the next output of the electrode.
    fn process(&mut self) -> f64;
}

/// Trait for any type of sensor.
pub trait Sensor {
    /// Returns the current output of the sensor.
    fn output(&self) -> f64;
}

/// Trait for any type of synapse.
pub trait Synapse {
    /// Returns the synaptic ID.
    fn syn_id(&self) -> SynapticID;

    /// Returns the synaptic type.
    fn syn_type(&self) -> SynapseType;

    /// Returns the presynaptic input.
    fn x_pre(&self) -> PresynapticInput;

    /// Returns the current total synaptic input.
    fn input(&self) -> f64;

    /// Processes and returns the next total input of the synapse.
    fn process(&mut self) -> f64;
}

// -------------------------------------------------------------------------------------------------

/// Contains data for short-term synaptic plasticity.
#[derive(Clone, Debug, PartialEq)]
pub struct ShortTermPlasticity {
    /// Short-term facilitation scalar.
    f: f64,
    /// Short-term facilitation recovery time.
    tf: f64,
    /// Short-term depression scalar.
    d: f64,
    /// Short-term depression decay time.
    td: f64,
    /// Short-term plasticity increment/decrement value.
    p: f64,
    /// Short-term plasticity weight output.
    y: f64
}

// ShortTermPlasticity functions
impl ShortTermPlasticity {
    /// Creates new short-term plasticity data.
    pub fn new(tf: f64, td: f64, p: f64) -> Self {
        Self {
            f: 1.0,
            tf,
            d: 0.0,
            td,
            p,
            y: 0.0
        }
    }

    /// Processes the short-term plasticity based on the specified spike input and returns the new short-term
    /// plasticity weight output.
    pub fn learn(&mut self, spike: bool) -> f64 {
        let pd = self.d;

        if spike {
            self.f += self.p * (1.0 - self.f);
            self.d -= self.f * self.d;
        }
        else {
            self.f -= self.f / self.tf;
            self.d += (1.0 - self.d) / self.td;
        }

        self.y = self.f * pd;
        self.y
    }
}

// -------------------------------------------------------------------------------------------------

/// Contains data for a pulse electrode.
#[derive(Clone, Debug, PartialEq)]
pub struct PulseElectrode {
    /// Active electrode flag.
    active: bool,
    /// The length of time the electrode has been active.
    a: f64,
    /// The electrode's voltage.
    v: f64,
    /// The duration of the pulse.
    d: f64,
    /// Current output of the electrode.
    y: f64
}

// Electrode functions
impl Electrode for PulseElectrode {
    /// Turns on (activates) the electrode.
    fn on(&mut self) {
        self.active = true;
    }

    /// Turns off (deactivates) the electrode.
    fn off(&mut self) {
        self.active = false;
    }

    /// Returns the voltage input of the electrode.
    fn voltage(&self) -> f64 {
        self.v
    }

    /// Returns the duration of the electrode's pulse.
    fn duration(&self) -> f64 {
        self.d
    }

    /// Returns the current output of the electrode.
    fn output(&self) -> f64 {
        self.y
    }

    /// Processes and returns the next output of the electrode.
    fn process(&mut self) -> f64 {
        if self.active {
            if self.a <= self.d {
                self.a += TIME_INC;
                self.y = self.v;
                return self.v;
            }
            else {
                self.a = 0.0;
                self.active = false;
                self.y = 0.0;
                return 0.0;
            }
        }
        else {
            self.a = 0.0;
            self.y = 0.0;
            0.0
        }
    }
}

// PulseElectrode functions
impl PulseElectrode {
    /// Creates a new pulse electrode with the specified parameters.
    pub fn new(v: f64, d: f64) -> Self {
        Self {
            active: false,
            a: 0.0,
            v,
            d,
            y: 0.0
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Contains data for a pulsating electrode.
#[derive(Clone, Debug, PartialEq)]
pub struct PulsatingElectrode {
    /// Active electrode flag.
    active: bool,
    /// The length of time the electrode has been active.
    a: f64,
    /// The electrode's voltage.
    v: f64,
    /// The duration of the pulse.
    d: f64,
    /// The time between pulses.
    t: f64,
    /// Current output of the electrode.
    y: f64
}

// Electrode functions
impl Electrode for PulsatingElectrode {
    /// Turns on (activates) the electrode.
    fn on(&mut self) {
        self.active = true;
    }

    /// Turns off (deactivates) the electrode.
    fn off(&mut self) {
        self.active = false;
    }

    /// Returns the voltage input of the electrode.
    fn voltage(&self) -> f64 {
        self.v
    }

    /// Returns the duration of the electrode's pulse.
    fn duration(&self) -> f64 {
        self.d
    }

    /// Returns the current output of the electrode.
    fn output(&self) -> f64 {
        self.y
    }

    /// Processes and returns the next output of the electrode.
    fn process(&mut self) -> f64 {
        if self.active {
            self.a += TIME_INC;

            if self.a % (self.d + self.t) <= self.d {
                self.y = self.v;
                return self.v;
            }
            else {
                self.y = 0.0;
                return 0.0;
            }
        }
        else {
            self.a = 0.0;
            self.y = 0.0;
            0.0
        }
    }
}

// PulsatingElectrode functions
impl PulsatingElectrode {
    /// Creates a new pulsating electrode with the specified parameters.
    pub fn new(v: f64, d: f64, t: f64) -> Self {
        Self {
            active: false,
            a: 0.0,
            v,
            d,
            t,
            y: 0.0
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Contains data for a sinusoidal electrode.
#[derive(Clone, Debug, PartialEq)]
pub struct SinusoidalElectrode {
    /// Active electrode flag.
    active: bool,
    /// The length of time the electrode has been active.
    a: f64,
    /// The electrode's voltage.
    v: f64,
    /// The period length of the pulse.
    d: f64,
    /// The phase shift of the sine wave.
    p: f64,
    /// Current output of the electrode.
    y: f64
}

// Electrode functions
impl Electrode for SinusoidalElectrode {
    /// Turns on (activates) the electrode.
    fn on(&mut self) {
        self.active = true;
    }

    /// Turns off (deactivates) the electrode.
    fn off(&mut self) {
        self.active = false;
    }

    /// Returns the voltage input of the electrode.
    fn voltage(&self) -> f64 {
        self.v
    }

    /// Returns the duration of the electrode's pulse.
    fn duration(&self) -> f64 {
        self.d
    }

    /// Returns the current output of the electrode.
    fn output(&self) -> f64 {
        self.y
    }

    /// Processes and returns the next output of the electrode.
    fn process(&mut self) -> f64 {
        if self.active {
            self.a += TIME_INC;

            self.y = self.v * ((2.0 * PI * (1.0 / self.d) * (self.a - self.p)).sin() * 0.5 + 0.5);
            return self.y;
        }
        else {
            self.a = 0.0;
            self.y = 0.0;
            0.0
        }
    }
}

// SinusoidalElectrode functions
impl SinusoidalElectrode {
    /// Creates a new sinusoidal electrode with the specified parameters.
    pub fn new(v: f64, d: f64, p: f64) -> Self {
        Self {
            active: false,
            a: 0.0,
            v,
            d,
            p,
            y: 0.0
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Contains data for a poisson electrode.
#[derive(Clone, Debug, PartialEq)]
pub struct PoissonElectrode {
    /// Active electrode flag.
    active: bool,
    /// The length of time the electrode has been active.
    a: f64,
    /// The electrode's voltage.
    v: f64,
    /// The duration of the pulse.
    d: f64,
    /// The average pulse frequency.
    f: f64,
    /// The time length between pulses.
    t: f64,
    /// Current output of the electrode.
    y: f64
}

// Electrode functions
impl Electrode for PoissonElectrode {
    /// Turns on (activates) the electrode.
    fn on(&mut self) {
        self.active = true;
    }

    /// Turns off (deactivates) the electrode.
    fn off(&mut self) {
        self.active = false;
    }

    /// Returns the voltage input of the electrode.
    fn voltage(&self) -> f64 {
        self.v
    }

    /// Returns the duration of the electrode's pulse.
    fn duration(&self) -> f64 {
        self.d
    }

    /// Returns the current output of the electrode.
    fn output(&self) -> f64 {
        self.y
    }

    /// Processes and returns the next output of the electrode.
    fn process(&mut self) -> f64 {
        if self.active {
            let prev_a: f64 = self.a;
            self.a += TIME_INC;

            if prev_a == 0.0 || self.a % 1000.0 < prev_a {
                let r: f64 = random();
                let mut k: i128 = 0;
                let mut p: f64 = (-self.f).exp();

                while r > p {
                    k += 1;
                    let mut fact: i128 = 1;

                    for i in 1..=k {
                        fact *= i;
                    }

                    p += (self.f.pow(k) * (-self.f).exp()) / fact;
                }

                self.t = (1000.0 / k as f64) - 1.0;
            }

            if self.a % (self.d + self.t) <= self.d {
                self.y = self.v;
                return self.v;
            }
            else {
                self.y = 0.0;
                return 0.0;
            }
        }
        else {
            self.a = 0.0;
            self.y = 0.0;
            0.0
        }
    }
}

// PoissonElectrode functions
impl PoissonElectrode {
    /// Creates a new poisson electrode with the specified parameters.
    pub fn new(v: f64, f: f64) -> Self {
        Self {
            active: false,
            a: 0.0,
            v,
            d: 1.0,
            f,
            t: 0.0,
            y: 0.0
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Contains data for an excitatory synapse.
#[derive(Clone, Debug, PartialEq)]
pub struct ExcitatorySynapse {
    /// The synaptic ID.
    id: SynapticID,
    /// The type of synapse.
    syn_type: SynapseType,
    /// The presynaptic input.
    x_pre: PresynapticInput,
    /// The postsynaptic neurite.
    x_post: Rc<RefCell<Neurite>>,
    /// A synaptic modulatory synapse (optional) connected to this synapse.
    mod_syn: Option<Rc<RefCell<SynapticModulatorySynapse>>>,
    /// The input conductance of the synapse (total input).
    x: f64,
    /// Conductance decay time.
    tx: f64,
    /// Short-term plasticity data.
    stp: ShortTermPlasticity,
    /// Synaptic weight.
    w: f64,
    /// Maximal synaptic input conductance (maximal input).
    x_max: f64
}

// Synapse functions for ExcitatorySynapse
impl Synapse for ExcitatorySynapse {
    /// Returns the synaptic ID.
    fn syn_id(&self) -> SynapticID {
        self.id
    }

    /// Returns the synaptic type.
    fn syn_type(&self) -> SynapseType {
        self.syn_type
    }

    /// Returns the presynaptic input.
    fn x_pre(&self) -> PresynapticInput {
        self.x_pre.clone()
    }

    /// Returns the current total synaptic input.
    fn input(&self) -> f64 {
        self.x
    }

    /// Processes and returns the next total input of the synapse.
    fn process(&mut self) -> f64 {
        self.x += -self.x / self.tx;

        match self.x_pre.clone() {
            PresynapticInput::Electrode(e) => {
                self.stp.learn(e.borrow().output() > 0.0);
            }
            PresynapticInput::Sensor(s) => {
                self.stp.learn(s.borrow().output() > 0.0);
            }
            PresynapticInput::Neurite(n) => {
                self.stp.learn(n.borrow().y > 0.0);
            }
        }

        self.x += self.x_max * self.stp.y * self.w;

        self.x
    }
}

// ExcitatorySynapse functions
impl ExcitatorySynapse {
    /// Creates a new excitatory synapse with the specified parameters.
    pub fn new(x_pre: PresynapticInput, x_post: Rc<RefCell<Neurite>>, stp: ShortTermPlasticity, tx: f64, x_max: f64) -> Self {
        Self {
            id: SynapticID::new(),
            syn_type: SynapseType::Excitatory,
            x_pre,
            x_post,
            mod_syn: None,
            x: 0.0,
            tx,
            stp,
            w: 0.0,
            x_max
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Contains data for an inhibitory synapse.
#[derive(Clone, Debug, PartialEq)]
pub struct InhibitorySynapse {
    /// The synaptic ID.
    id: SynapticID,
    /// The type of synapse.
    syn_type: SynapseType,
    /// The presynaptic input.
    x_pre: PresynapticInput,
    /// The postsynaptic neurite.
    x_post: Rc<RefCell<Neurite>>,
    /// A synaptic modulatory synapse (optional) connected to this synapse.
    mod_syn: Option<Rc<RefCell<SynapticModulatorySynapse>>>,
    /// The input conductance of the synapse (total input).
    x: f64,
    /// Conductance decay time.
    tx: f64,
    /// Short-term plasticity data.
    stp: ShortTermPlasticity,
    /// Synaptic weight.
    w: f64,
    /// Maximal synaptic input conductance (maximal input).
    x_max: f64
}

// InhibitorySynapse functions
impl InhibitorySynapse {
    /// Creates a new fast inhibitory synapse with the specified parameters.
    pub fn new(x_pre: PresynapticInput, x_post: Rc<RefCell<Neurite>>, stp: ShortTermPlasticity, tx: f64, x_max: f64) -> Self {
        Self {
            id: SynapticID::new(),
            syn_type: SynapseType::Inhibitory,
            x_pre,
            x_post,
            mod_syn: None,
            x: 0.0,
            tx,
            stp,
            w: 0.0,
            x_max
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Contains data for a shunting inhibitory synapse.
#[derive(Clone, Debug, PartialEq)]
pub struct ShuntingInhibitorySynapse {
    /// The synaptic ID.
    id: SynapticID,
    /// The type of synapse.
    syn_type: SynapseType,
    /// The presynaptic input.
    x_pre: PresynapticInput,
    /// The postsynaptic neurite.
    x_post: Rc<RefCell<Neurite>>,
    /// A synaptic modulatory synapse (optional) connected to this synapse.
    mod_syn: Option<Rc<RefCell<SynapticModulatorySynapse>>>,
    /// The input conductance of the synapse (total input).
    x: f64,
    /// Conductance decay time.
    tx: f64,
    /// The shunting scalar (-2..1).
    s: f64,
    /// Short-term plasticity data.
    stp: ShortTermPlasticity,
    /// Synaptic weight.
    w: f64,
    /// Maximal synaptic input conductance (maximal input).
    x_max: f64
}

// ShuntingInhibitorySynapse functions
impl ShuntingInhibitorySynapse {
    /// Creates a new shunting inhibitory synapse with the specified parameters.
    pub fn new(x_pre: PresynapticInput, x_post: Rc<RefCell<Neurite>>, stp: ShortTermPlasticity, s: f64, tx: f64, x_max: f64) -> Self {
        Self {
            id: SynapticID::new(),
            syn_type: SynapseType::ShuntingInhibitory,
            x_pre,
            x_post,
            mod_syn: None,
            x: 0.0,
            tx,
            s: 1.0.min(-2.0.max(s)),
            stp,
            w: 0.0,
            x_max
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Contains data for a synaptic modulatory synapse.
#[derive(Clone, Debug, PartialEq)]
pub struct SynapticModulatorySynapse {
    /// The synaptic ID.
    id: SynapticID,
    /// The type of synapse.
    syn_type: SynapseType,
    /// The presynaptic input.
    x_pre: PresynapticInput,
    /// The postsynaptic neurite.
    x_post: Rc<RefCell<Neurite>>,
    /// A synaptic modulatory synapse (optional) connected to this synapse.
    mod_syn: Option<Rc<RefCell<SynapticModulatorySynapse>>>,
    /// The input conductance of the synapse (total input).
    x: f64,
    /// Conductance decay time.
    tx: f64,
    /// Short-term plasticity data.
    stp: ShortTermPlasticity
}

// SynapticModulatorySynapse functions
impl SynapticModulatorySynapse {
    /// Creates a new synaptic modulatory synapse with the specified parameters.
    pub fn new(var: SynapticModVar, x_pre: PresynapticInput, x_post: Rc<RefCell<Neurite>>, stp: ShortTermPlasticity, tx: f64) -> Self {
        Self {
            id: SynapticID::new(),
            syn_type: SynapseType::SynapticModulator(var),
            x_pre,
            x_post,
            mod_syn: None,
            x: 0.0,
            tx,
            stp
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Contains data for a neural modulatory synapse.
#[derive(Clone, Debug, PartialEq)]
pub struct NeuralModulatorySynapse {
    /// The synaptic ID.
    id: SynapticID,
    /// The type of synapse.
    syn_type: SynapseType,
    /// The presynaptic input.
    x_pre: PresynapticInput,
    /// The postsynaptic neurite.
    x_post: Rc<RefCell<Neurite>>,
    /// A synaptic modulatory synapse (optional) connected to this synapse.
    mod_syn: Option<Rc<RefCell<SynapticModulatorySynapse>>>,
    /// The input conductance of the synapse (total input).
    x: f64,
    /// Conductance decay time.
    tx: f64,
    /// Short-term plasticity data.
    stp: ShortTermPlasticity
}

// NeuralModulatorySynapse functions
impl NeuralModulatorySynapse {
    /// Creates a new neural modulatory synapse with the specified parameters.
    pub fn new(var: NeuriteModVar, x_pre: PresynapticInput, x_post: Rc<RefCell<Neurite>>, stp: ShortTermPlasticity, tx: f64) -> Self {
        Self {
            id: SynapticID::new(),
            syn_type: SynapseType::NeuralModulator(var),
            x_pre,
            x_post,
            mod_syn: None,
            x: 0.0,
            tx,
            stp
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Contains data for a gap junction synapse.
#[derive(Clone, Debug, PartialEq)]
pub struct GapJunctionSynapse {
    /// The synaptic ID.
    id: SynapticID,
    /// The type of synapse.
    syn_type: SynapseType,
    /// The presynaptic input.
    x_pre: PresynapticInput,
    /// The postsynaptic neurite.
    x_post: Rc<RefCell<Neurite>>,
    /// A synaptic modulatory synapse (optional) connected to this synapse.
    mod_syn: Option<Rc<RefCell<SynapticModulatorySynapse>>>,
    /// The conductance of the synapse from input a.
    ax: f64,
    /// The conductance of the synapse from input b.
    bx: f64,
    /// Short-term plasticity data.
    stp: ShortTermPlasticity,
    /// Synaptic weight.
    w: f64,
    /// Maximal synaptic input conductance (maximal input).
    x_max: f64
}

// GapJunctionSynapse functions
impl GapJunctionSynapse {
    /// Creates a new gap junction synapse with the specified parameters.
    pub fn new(x_pre: PresynapticInput, x_post: Rc<RefCell<Neurite>>, stp: ShortTermPlasticity, x_max: f64) -> Self {
        Self {
            id: SynapticID::new(),
            syn_type: SynapseType::Gap,
            x_pre,
            x_post,
            mod_syn: None,
            ax: 0.0,
            bx: 0.0,
            stp,
            w: 0.0,
            x_max
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Contains the neurite ID value for a neurite.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NeuriteID {
    id: Uuid
}

// NeuriteID function
impl NeuriteID {
    /// Creates a new unique neurite ID value.
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v7(Timestamp::now(SystemTime::now()))
        }
    }
}

/// Contains a list of neurite types.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NeuriteType {
    Soma,
    BasalProximal,
    BasalDistal,
    ApicalTrunk,
    ApicalTuft,
    Axon
}

/// Contains a list of spike models.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SpikeModel {
    Accommodation,
    Bistability,
    ChatteringI,
    ChatteringII,
    ClassI,
    ClassII,
    DepolarizingAfterPotential,
    EntorhinalStellate,
    FastSpiking,
    FastSpikingBasket,
    HippocampalCA1PyramidalHighThresholdBursting,
    HippocampalCA1PyramidalLowThresholdBurstingI,
    HippocampalCA1PyramidalLowThresholdBurstingII,
    HippocampalCA1PyramidalNonBursting,
    InhibitionInducedBursting,
    InhibitionInducedSpiking,
    Integrator,
    IntrinsicallyBurstingPyramidal,
    IntrinsicallyBurstingPyramidalDendriteI,
    IntrinsicallyBurstingPyramidalDendriteII,
    IntrinsicallyBurstingPyramidalSomaI,
    IntrinsicallyBurstingPyramidalSomaII,
    LatentSpikingNonBasket,
    LatentSpikingNonBasketDendrite,
    LowThresholdSpiking,
    LowThresholdSpikingNonBasket,
    MixedMode,
    PhasicBursting,
    PhasicSpiking,
    ReboundBurst,
    ReboundSpike,
    RegularSpiking,
    RegularSpikingPyramidalI,
    RegularSpikingPyramidalII,
    RegularSpikingPyramidalL2L3Dendrite,
    RegularSpikingPyramidalL4Dendrite,
    RegularSpikingPyramidalL5L6Dendrite,
    RegularSpikingSpinyStellate,
    RegularSpikingSpinyStellateDendrite,
    ResonatorI,
    ResonatorII,
    ReticularThalamicNeuron,
    SpikeFrequencyAdaptation,
    SpikeLatency,
    SpinyProjection,
    SubthresholdOscillation,
    ThalamicInterneuron,
    Thalamocortical,
    ThalamocorticalBursting,
    ThalamocorticalSpiking,
    ThresholdVariability,
    TonicBursting,
    TonicSpiking,
}

// -------------------------------------------------------------------------------------------------

/// Contains data for a neurite (neuron compartment).
#[derive(Clone, Debug, PartialEq)]
pub struct Neurite {
    /// The neurite ID value.
    id: NeuriteID,
    /// The neurite type.
    neurite_type: NeuriteType,
    /// The spike model.
    spike_model: SpikeModel,
    /// Timescale for recovery variable u.
    a: f64,
    /// Sensitivity of recovery variable u to sub-threshold oscillations.
    b: f64,
    /// After-spike reset value of the membrane potential v.
    c: f64,
    /// After-spike reset value of recovery variable u.
    d: f64,
    /// Membrane potential recovery variable.
    u: f64,
    /// Membrane potential/voltage.
    v: f64,
    /// Total membrane potential of child compartment(s).
    vcc: f64,
    /// Membrane potential of parent compartment.
    vpc: f64,
    /// Spike output.
    y: f64,
    /// Neurite's extended variables.
    ext: Option<NeuriteExt>,
    /// Neurite's synapses.
    syn: Vec<Rc<RefCell<dyn Synapse>>>,
    /// Parent neurite.
    parent: Option<Rc<RefCell<Neurite>>>,
    /// Neurite's children.
    child: Vec<Rc<RefCell<Neurite>>>
}

/// Contains data for an extended neurite (neuron compartment).
#[derive(Clone, Debug, PartialEq)]
pub struct NeuriteExt {
    /// Positive scalar value.
    k: f64,
    /// Conductance of child compartment(s).
    gcc: f64,
    /// Conductance of parent compartment.
    gpc: f64,
    /// Resting membrane potential.
    vr: f64,
    /// Instantaneous threshold potential.
    vt: f64,
    /// Spike peak membrane potential.
    vp: f64,
    /// Membrane capacitance.
    cap: f64,
}

// Deref function for NeuriteExt
impl Deref for NeuriteExt {
    type Target = NeuriteExt;

    fn deref(&self) -> &Self::Target {
        self
    }
}

// DerefMut function for NeuriteExt
impl DerefMut for NeuriteExt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self
    }
}

// Neurite functions
impl Neurite {
    /// Creates a new neurite with the specified spike model. If extended spike model is specified, the spike model defaults to a
    /// regular spiking model.
    pub fn new(neurite_type: NeuriteType, spike_model: SpikeModel) -> Self {
        return match spike_model {
            SpikeModel::Accommodation => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.02,
                    b: 1.0,
                    c: -55.0,
                    d: 4.0,
                    u: -16.0,
                    v: -65.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: None,
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::Bistability => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.1,
                    b: 0.26,
                    c: -60.0,
                    d: 0.0,
                    u: -15.86,
                    v: -61.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: None,
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::ChatteringI | SpikeModel::TonicBursting => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.02,
                    b: 0.2,
                    c: -50.0,
                    d: 2.0,
                    u: -14.0,
                    v: -70.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: None,
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::ChatteringII => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.03,
                    b: 1.0,
                    c: -40.0,
                    d: 150.0,
                    u: 0.0,
                    v: -60.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: Some(NeuriteExt {
                        k: 1.5,
                        gcc: 0.0,
                        gpc: 0.0,
                        vr: -60.0,
                        vt: -40.0,
                        vp: 25.0,
                        cap: 50.0,
                    }),
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::ClassI | SpikeModel::Integrator => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.02,
                    b: -0.1,
                    c: -55.0,
                    d: 6.0,
                    u: 6.0,
                    v: -60.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: None,
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::ClassII => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.2,
                    b: 0.26,
                    c: -65.0,
                    d: 0.0,
                    u: -16.64,
                    v: -64.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: None,
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::DepolarizingAfterPotential => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 1.0,
                    b: 0.2,
                    c: -60.0,
                    d: -21.0,
                    u: -14.0,
                    v: -70.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: None,
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::EntorhinalStellate => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.01,
                    b: 15.0,
                    c: -50.0,
                    d: 0.0,
                    u: 0.0,
                    v: -60.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: Some(NeuriteExt {
                        k: 0.75,
                        gcc: 1.0,
                        gpc: 1.0,
                        vr: -60.0,
                        vt: -45.0,
                        vp: 30.0,
                        cap: 200.0,
                    }),
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::FastSpiking => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.1,
                    b: 0.2,
                    c: -65.0,
                    d: 2.0,
                    u: -14.0,
                    v: -70.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: None,
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::FastSpikingBasket => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.2,
                    b: 0.0,
                    c: -55.0,
                    d: 0.0,
                    u: 0.0,
                    v: -55.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: Some(NeuriteExt {
                        k: 1.0,
                        gcc: 0.5,
                        gpc: 1.0,
                        vr: -55.0,
                        vt: -40.0,
                        vp: 25.0,
                        cap: 20.0,
                    }),
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::HippocampalCA1PyramidalHighThresholdBursting => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.02,
                    b: 0.5,
                    c: -45.0,
                    d: 50.0,
                    u: 0.0,
                    v: -60.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: Some(NeuriteExt {
                        k: 1.0,
                        gcc: 1.0,
                        gpc: 1.0,
                        vr: -60.0,
                        vt: -45.0,
                        vp: 40.0,
                        cap: 50.0,
                    }),
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::HippocampalCA1PyramidalLowThresholdBurstingI => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.02,
                    b: 0.5,
                    c: -40.0,
                    d: 55.0,
                    u: 0.0,
                    v: -60.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: Some(NeuriteExt {
                        k: 1.0,
                        gcc: 1.0,
                        gpc: 1.0,
                        vr: -60.0,
                        vt: -45.0,
                        vp: 40.0,
                        cap: 50.0,
                    }),
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::HippocampalCA1PyramidalLowThresholdBurstingII => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.02,
                    b: 0.5,
                    c: -35.0,
                    d: 60.0,
                    u: 0.0,
                    v: -60.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: Some(NeuriteExt {
                        k: 1.0,
                        gcc: 1.0,
                        gpc: 1.0,
                        vr: -60.0,
                        vt: -45.0,
                        vp: 40.0,
                        cap: 50.0,
                    }),
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::HippocampalCA1PyramidalNonBursting => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.02,
                    b: 0.5,
                    c: -50.0,
                    d: 50.0,
                    u: 0.0,
                    v: -60.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: Some(NeuriteExt {
                        k: 1.0,
                        gcc: 1.0,
                        gpc: 1.0,
                        vr: -60.0,
                        vt: -45.0,
                        vp: 40.0,
                        cap: 50.0,
                    }),
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::InhibitionInducedBursting => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.026,
                    b: -1.0,
                    c: -45.0,
                    d: -2.0,
                    u: 63.8,
                    v: -63.8,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: None,
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::InhibitionInducedSpiking => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.02,
                    b: -1.0,
                    c: -60.0,
                    d: 8.0,
                    u: 63.8,
                    v: -63.8,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: None,
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::IntrinsicallyBurstingPyramidal => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.01,
                    b: 5.0,
                    c: -56.0,
                    d: 130.0,
                    u: 0.0,
                    v: -75.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: Some(NeuriteExt {
                        k: 1.2,
                        gcc: 1.0,
                        gpc: 1.0,
                        vr: -75.0,
                        vt: -45.0,
                        vp: 50.0,
                        cap: 150.0,
                    }),
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::IntrinsicallyBurstingPyramidalDendriteI => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 3.0,
                    b: 15.0,
                    c: -20.0,
                    d: 500.0,
                    u: 0.0,
                    v: -50.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: Some(NeuriteExt {
                        k: 1.0,
                        gcc: 1.0,
                        gpc: 1.0,
                        vr: -50.0,
                        vt: -50.0,
                        vp: 20.0,
                        cap: 30.0,
                    }),
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::IntrinsicallyBurstingPyramidalDendriteII => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.01,
                    b: 5.0,
                    c: -35.0,
                    d: 1000.0,
                    u: 0.0,
                    v: -60.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: Some(NeuriteExt {
                        k: 3.0,
                        gcc: 0.007,
                        gpc: 0.007,
                        vr: -60.0,
                        vt: -50.0,
                        vp: 10.0,
                        cap: 100.0,
                    }),
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::IntrinsicallyBurstingPyramidalSomaI => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.01,
                    b: 5.0,
                    c: -52.0,
                    d: 240.0,
                    u: 0.0,
                    v: -70.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: Some(NeuriteExt {
                        k: 3.0,
                        gcc: 1.0,
                        gpc: 1.0,
                        vr: -70.0,
                        vt: -45.0,
                        vp: 50.0,
                        cap: 150.0,
                    }),
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::IntrinsicallyBurstingPyramidalSomaII => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.01,
                    b: 5.0,
                    c: -55.0,
                    d: 500.0,
                    u: 0.0,
                    v: -60.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: Some(NeuriteExt {
                        k: 3.0,
                        gcc: 0.007,
                        gpc: 0.007,
                        vr: -60.0,
                        vt: -50.0,
                        vp: 50.0,
                        cap: 100.0,
                    }),
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::LatentSpikingNonBasket => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.17,
                    b: 5.0,
                    c: -45.0,
                    d: 20.0,
                    u: 0.0,
                    v: -53.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: Some(NeuriteExt {
                        k: 0.3,
                        gcc: 0.6,
                        gpc: 2.5,
                        vr: -66.0,
                        vt: -40.0,
                        vp: 30.0,
                        cap: 20.0,
                    }),
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::LatentSpikingNonBasketDendrite => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.17,
                    b: 5.0,
                    c: -45.0,
                    d: 20.0,
                    u: 0.0,
                    v: -53.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: Some(NeuriteExt {
                        k: 0.3,
                        gcc: 0.6,
                        gpc: 2.5,
                        vr: -66.0,
                        vt: -40.0,
                        vp: 100.0,
                        cap: 20.0,
                    }),
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::LowThresholdSpiking => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.02,
                    b: 0.25,
                    c: -65.0,
                    d: 2.0,
                    u: -15.75,
                    v: -63.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: None,
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::LowThresholdSpikingNonBasket => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.03,
                    b: 8.0,
                    c: -53.0,
                    d: 20.0,
                    u: 0.0,
                    v: -53.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: Some(NeuriteExt {
                        k: 3.0,
                        gcc: 1.0,
                        gpc: 1.0,
                        vr: -56.0,
                        vt: -42.0,
                        vp: 40.0,
                        cap: 100.0,
                    }),
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::MixedMode => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.02,
                    b: 0.2,
                    c: -55.0,
                    d: 4.0,
                    u: -14.0,
                    v: -70.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: None,
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::PhasicBursting => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.02,
                    b: 0.25,
                    c: -55.0,
                    d: 0.05,
                    u: -16.0,
                    v: -64.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: None,
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::PhasicSpiking => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.02,
                    b: 0.25,
                    c: -65.0,
                    d: 6.0,
                    u: -16.0,
                    v: -64.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: None,
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::ReboundBurst => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.03,
                    b: 0.25,
                    c: -52.0,
                    d: 0.0,
                    u: -16.0,
                    v: -64.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: None,
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::ReboundSpike | SpikeModel::ThresholdVariability => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.03,
                    b: 0.25,
                    c: -60.0,
                    d: 4.0,
                    u: -16.0,
                    v: -64.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: None,
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::RegularSpiking => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.02,
                    b: 0.2,
                    c: -65.0,
                    d: 8.0,
                    u: -12.6,
                    v: -63.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: None,
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::RegularSpikingPyramidalI => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.03,
                    b: -2.0,
                    c: -50.0,
                    d: 100.0,
                    u: 0.0,
                    v: -60.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: Some(NeuriteExt {
                        k: 0.7,
                        gcc: 1.0,
                        gpc: 1.0,
                        vr: -60.0,
                        vt: -40.0,
                        vp: 35.0,
                        cap: 100.0,
                    }),
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::RegularSpikingPyramidalII | SpikeModel::RegularSpikingSpinyStellate => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.01,
                    b: 5.0,
                    c: -60.0,
                    d: 400.0,
                    u: 0.0,
                    v: -60.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: Some(NeuriteExt {
                        k: 3.0,
                        gcc: 3.0,
                        gpc: 5.0,
                        vr: -60.0,
                        vt: -50.0,
                        vp: 50.0,
                        cap: 100.0,
                    }),
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::RegularSpikingPyramidalL2L3Dendrite => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.01,
                    b: 5.0,
                    c: -55.0,
                    d: 400.0,
                    u: 0.0,
                    v: -60.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: Some(NeuriteExt {
                        k: 3.0,
                        gcc: 3.0,
                        gpc: 5.0,
                        vr: -60.0,
                        vt: -50.0,
                        vp: 30.0,
                        cap: 100.0,
                    }),
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::RegularSpikingPyramidalL4Dendrite => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.01,
                    b: 5.0,
                    c: -50.0,
                    d: 400.0,
                    u: 0.0,
                    v: -60.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: Some(NeuriteExt {
                        k: 3.0,
                        gcc: 3.0,
                        gpc: 5.0,
                        vr: -60.0,
                        vt: -50.0,
                        vp: 50.0,
                        cap: 100.0,
                    }),
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::RegularSpikingPyramidalL5L6Dendrite | SpikeModel::RegularSpikingSpinyStellateDendrite => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.01,
                    b: 5.0,
                    c: -50.0,
                    d: 400.0,
                    u: 0.0,
                    v: -60.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: Some(NeuriteExt {
                        k: 3.0,
                        gcc: 3.0,
                        gpc: 5.0,
                        vr: -60.0,
                        vt: -50.0,
                        vp: 30.0,
                        cap: 100.0,
                    }),
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::ResonatorI => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.1,
                    b: 0.26,
                    c: -65.0,
                    d: 2.0,
                    u: -18.2,
                    v: -70.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: None,
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::ResonatorII => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.1,
                    b: 0.26,
                    c: -60.0,
                    d: -1.0,
                    u: -16.12,
                    v: -62.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: None,
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::ReticularThalamicNeuron => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.015,
                    b: 10.0,
                    c: -55.0,
                    d: 50.0,
                    u: 0.0,
                    v: 0.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: Some(NeuriteExt {
                        k: 0.25,
                        gcc: 5.0,
                        gpc: 5.0,
                        vr: -65.0,
                        vt: -45.0,
                        vp: 0.0,
                        cap: 40.0,
                    }),
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::SpikeFrequencyAdaptation => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.01,
                    b: 0.2,
                    c: -65.0,
                    d: 8.0,
                    u: -14.0,
                    v: -70.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: None,
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::SpikeLatency | SpikeModel::TonicSpiking => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.02,
                    b: 0.2,
                    c: -65.0,
                    d: 6.0,
                    u: -14.0,
                    v: -70.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: None,
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::SpinyProjection => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.01,
                    b: -20.0,
                    c: -55.0,
                    d: 150.0,
                    u: 0.0,
                    v: -80.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: Some(NeuriteExt {
                        k: 1.0,
                        gcc: 1.0,
                        gpc: 1.0,
                        vr: -80.0,
                        vt: -25.0,
                        vp: 40.0,
                        cap: 50.0,
                    }),
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::SubthresholdOscillation => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.05,
                    b: 0.26,
                    c: -60.0,
                    d: 0.0,
                    u: -16.12,
                    v: -62.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: None,
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::ThalamicInterneuron => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.05,
                    b: 7.0,
                    c: -65.0,
                    d: 50.0,
                    u: 0.0,
                    v: -60.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: Some(NeuriteExt {
                        k: 0.5,
                        gcc: 5.0,
                        gpc: 5.0,
                        vr: -60.0,
                        vt: -50.0,
                        vp: 20.0,
                        cap: 20.0,
                    }),
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::Thalamocortical => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.1,
                    b: 15.0,
                    c: -60.0,
                    d: 10.0,
                    u: 0.0,
                    v: -60.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: Some(NeuriteExt {
                        k: 1.6,
                        gcc: 2.0,
                        gpc: 2.0,
                        vr: -60.0,
                        vt: -50.0,
                        vp: 40.0,
                        cap: 200.0,
                    }),
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::ThalamocorticalSpiking => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.02,
                    b: 0.25,
                    c: -65.0,
                    d: 0.05,
                    u: -15.75,
                    v: -63.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: None,
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
            SpikeModel::ThalamocorticalBursting => {
                Self {
                    id: NeuriteID::new(),
                    neurite_type,
                    spike_model,
                    a: 0.02,
                    b: 0.25,
                    c: -65.0,
                    d: 0.05,
                    u: -21.75,
                    v: -87.0,
                    vcc: 0.0,
                    vpc: 0.0,
                    y: 0.0,
                    ext: None,
                    syn: vec![],
                    parent: None,
                    child: vec![]
                }
            }
        }
    }

    /// Sets the spike model. If the spike model is set to extended model, it will be defaulted to a regular spiking model.
    pub fn set_spike_model(&mut self, spike_model: SpikeModel) {
        match spike_model {
            SpikeModel::Accommodation => {
                self.spike_model = spike_model;
                self.a = 0.02;
                self.b = 1.0;
                self.c = -55.0;
                self.d = 4.0;
                self.u = -16.0;
                self.v = -65.0;
                self.y = 0.0;
                self.ext = None;
            }
            SpikeModel::Bistability => {
                self.spike_model = spike_model;
                self.a = 0.1;
                self.b = 0.26;
                self.c = -60.0;
                self.d = 0.0;
                self.u = -15.86;
                self.v = -61.0;
                self.y = 0.0;
                self.ext = None;
            }
            SpikeModel::ChatteringI => {
                self.spike_model = spike_model;
                self.a = 0.02;
                self.b = 0.2;
                self.c = -50.0;
                self.d = 2.0;
                self.u = -14.0;
                self.v = -70.0;
                self.y = 0.0;
                self.ext = None;
            }
            SpikeModel::ChatteringII => {
                self.spike_model = spike_model;
                self.a = 0.03;
                self.b = 1.0;
                self.c = -40.0;
                self.d = 150.0;
                self.u = 0.0;
                self.v = -60.0;
                self.y = 0.0;
                self.ext = Some(NeuriteExt {
                    k: 1.5,
                    gcc: 0.0,
                    gpc: 0.0,
                    vr: -60.0,
                    vt: -40.0,
                    vp: 25.0,
                    cap: 50.0,
                });
            }
            SpikeModel::ClassI => {
                self.spike_model = spike_model;
                self.a = 0.02;
                self.b = 0.1;
                self.c = -55.0;
                self.d = 6.0;
                self.u = 6.0;
                self.v = -60.0;
                self.y = 0.0;
                self.ext = None;
            }
            SpikeModel::ClassII => {
                self.spike_model = spike_model;
                self.a = 0.2;
                self.b = 0.26;
                self.c = -65.0;
                self.d = 0.0;
                self.u = -16.64;
                self.v = -64.0;
                self.y = 0.0;
                self.ext = None;
            }
            SpikeModel::DepolarizingAfterPotential => {
                self.spike_model = spike_model;
                self.a = 1.0;
                self.b = 0.2;
                self.c = -60.0;
                self.d = -21.0;
                self.u = -14.0;
                self.v = -70.0;
                self.y = 0.0;
                self.ext = None;
            }
            SpikeModel::EntorhinalStellate => {
                self.spike_model = spike_model;
                self.a = 0.01;
                self.b = 15.0;
                self.c = -50.0;
                self.d = 0.0;
                self.u = 0.0;
                self.v = -60.0;
                self.y = 0.0;
                self.ext = Some(NeuriteExt {
                    k: 0.75,
                    gcc: 1.0,
                    gpc: 1.0,
                    vr: -60.0,
                    vt: -45.0,
                    vp: 30.0,
                    cap: 200.0,
                });
            }
            SpikeModel::FastSpiking => {
                self.spike_model = spike_model;
                self.a = 0.1;
                self.b = 0.2;
                self.c = -65.0;
                self.d = 2.0;
                self.u = -14.0;
                self.v = -70.0;
                self.y = 0.0;
                self.ext = None;
            }
            SpikeModel::FastSpikingBasket => {
                self.spike_model = spike_model;
                self.a = 0.2;
                self.b = 0.0;
                self.c = -55.0;
                self.d = 0.0;
                self.u = 0.0;
                self.v = -55.0;
                self.y = 0.0;
                self.ext = Some(NeuriteExt {
                    k: 1.0,
                    gcc: 0.5,
                    gpc: 1.0,
                    vr: -55.0,
                    vt: -40.0,
                    vp: 25.0,
                    cap: 20.0,
                });
            }
            SpikeModel::HippocampalCA1PyramidalHighThresholdBursting => {
                self.spike_model = spike_model;
                self.a = 0.02;
                self.b = 0.5;
                self.c = -45.0;
                self.d = 50.0;
                self.u = 0.0;
                self.v = -60.0;
                self.y = 0.0;
                self.ext = Some(NeuriteExt {
                    k: 1.0,
                    gcc: 1.0,
                    gpc: 1.0,
                    vr: -60.0,
                    vt: -45.0,
                    vp: 40.0,
                    cap: 50.0,
                });
            }
            SpikeModel::HippocampalCA1PyramidalLowThresholdBurstingI => {
                self.spike_model = spike_model;
                self.a = 0.02;
                self.b = 0.5;
                self.c = -40.0;
                self.d = 55.0;
                self.u = 0.0;
                self.v = -60.0;
                self.y = 0.0;
                self.ext = Some(NeuriteExt {
                    k: 1.0,
                    gcc: 1.0,
                    gpc: 1.0,
                    vr: -60.0,
                    vt: -45.0,
                    vp: 40.0,
                    cap: 50.0,
                });
            }
            SpikeModel::HippocampalCA1PyramidalLowThresholdBurstingII => {
                self.spike_model = spike_model;
                self.a = 0.02;
                self.b = 0.5;
                self.c = -35.0;
                self.d = 60.0;
                self.u = 0.0;
                self.v = -60.0;
                self.y = 0.0;
                self.ext = Some(NeuriteExt {
                    k: 1.0,
                    gcc: 1.0,
                    gpc: 1.0,
                    vr: -60.0,
                    vt: -45.0,
                    vp: 40.0,
                    cap: 50.0,
                });
            }
            SpikeModel::HippocampalCA1PyramidalNonBursting => {
                self.spike_model = spike_model;
                self.a = 0.02;
                self.b = 0.5;
                self.c = -50.0;
                self.d = 50.0;
                self.u = 0.0;
                self.v = -60.0;
                self.y = 0.0;
                self.ext = Some(NeuriteExt {
                    k: 1.0,
                    gcc: 1.0,
                    gpc: 1.0,
                    vr: -60.0,
                    vt: -45.0,
                    vp: 40.0,
                    cap: 50.0,
                });
            }
            SpikeModel::InhibitionInducedBursting => {
                self.spike_model = spike_model;
                self.a = 0.026;
                self.b = -1.0;
                self.c = -45.0;
                self.d = -2.0;
                self.u = 63.8;
                self.v = -63.8;
                self.y = 0.0;
                self.ext = None;
            }
            SpikeModel::InhibitionInducedSpiking => {
                self.spike_model = spike_model;
                self.a = 0.02;
                self.b = -1.0;
                self.c = -60.0;
                self.d = 8.0;
                self.u = 63.8;
                self.v = -63.8;
                self.y = 0.0;
                self.ext = None;
            }
            SpikeModel::Integrator => {
                self.spike_model = spike_model;
                self.a = 0.02;
                self.b = -0.1;
                self.c = -55.0;
                self.d = 6.0;
                self.u = 6.0;
                self.v = -60.0;
                self.y = 0.0;
                self.ext = None;
            }
            SpikeModel::IntrinsicallyBurstingPyramidal => {
                self.spike_model = spike_model;
                self.a = 0.01;
                self.b = 5.0;
                self.c = -56.0;
                self.d = 130.0;
                self.u = 0.0;
                self.v = -75.0;
                self.y = 0.0;
                self.ext = Some(NeuriteExt {
                    k: 1.2,
                    gcc: 1.0,
                    gpc: 1.0,
                    vr: -75.0,
                    vt: -45.0,
                    vp: 50.0,
                    cap: 150.0,
                });
            }
            SpikeModel::IntrinsicallyBurstingPyramidalDendriteI => {
                self.spike_model = spike_model;
                self.a = 3.0;
                self.b = 15.0;
                self.c = -20.0;
                self.d = 500.0;
                self.u = 0.0;
                self.v = -50.0;
                self.y = 0.0;
                self.ext = Some(NeuriteExt {
                    k: 1.0,
                    gcc: 1.0,
                    gpc: 1.0,
                    vr: -50.0,
                    vt: -50.0,
                    vp: 20.0,
                    cap: 30.0,
                });
            }
            SpikeModel::IntrinsicallyBurstingPyramidalDendriteII => {
                self.spike_model = spike_model;
                self.a = 0.01;
                self.b = 5.0;
                self.c = -35.0;
                self.d = 1000.0;
                self.u = 0.0;
                self.v = -60.0;
                self.y = 0.0;
                self.ext = Some(NeuriteExt {
                    k: 3.0,
                    gcc: 0.7,
                    gpc: 0.7,
                    vr: -60.0,
                    vt: -50.0,
                    vp: 10.0,
                    cap: 100.0,
                });
            }
            SpikeModel::IntrinsicallyBurstingPyramidalSomaI => {
                self.spike_model = spike_model;
                self.a = 0.01;
                self.b = 5.0;
                self.c = -52.0;
                self.d = 240.0;
                self.u = 0.0;
                self.v = -70.0;
                self.y = 0.0;
                self.ext = Some(NeuriteExt {
                    k: 3.0,
                    gcc: 1.0,
                    gpc: 1.0,
                    vr: -70.0,
                    vt: -45.0,
                    vp: 50.0,
                    cap: 150.0,
                });
            }
            SpikeModel::IntrinsicallyBurstingPyramidalSomaII => {
                self.spike_model = spike_model;
                self.a = 0.01;
                self.b = 5.0;
                self.c = -55.0;
                self.d = 500.0;
                self.u = 0.0;
                self.v = -60.0;
                self.y = 0.0;
                self.ext = Some(NeuriteExt {
                    k: 3.0,
                    gcc: 0.7,
                    gpc: 0.7,
                    vr: -60.0,
                    vt: -50.0,
                    vp: 50.0,
                    cap: 100.0,
                });
            }
            SpikeModel::LatentSpikingNonBasket => {
                self.spike_model = spike_model;
                self.a = 0.17;
                self.b = 5.0;
                self.c = -45.0;
                self.d = 20.0;
                self.u = 0.0;
                self.v = -53.0;
                self.y = 0.0;
                self.ext = Some(NeuriteExt {
                    k: 0.3,
                    gcc: 0.6,
                    gpc: 2.5,
                    vr: -66.0,
                    vt: -40.0,
                    vp: 30.0,
                    cap: 20.0,
                });
            }
            SpikeModel::LatentSpikingNonBasketDendrite => {
                self.spike_model = spike_model;
                self.a = 0.17;
                self.b = 5.0;
                self.c = -45.0;
                self.d = 20.0;
                self.u = 0.0;
                self.v = -53.0;
                self.y = 0.0;
                self.ext = Some(NeuriteExt {
                    k: 0.3,
                    gcc: 0.6,
                    gpc: 2.5,
                    vr: -66.0,
                    vt: -40.0,
                    vp: 100.0,
                    cap: 20.0,
                });
            }
            SpikeModel::LowThresholdSpiking => {
                self.spike_model = spike_model;
                self.a = 0.02;
                self.b = 0.25;
                self.c = -65.0;
                self.d = 2.0;
                self.u = -15.75;
                self.v = -63.0;
                self.y = 0.0;
                self.ext = None;
            }
            SpikeModel::LowThresholdSpikingNonBasket => {
                self.spike_model = spike_model;
                self.a = 0.03;
                self.b = 8.0;
                self.c = -53.0;
                self.d = 20.0;
                self.u = 0.0;
                self.v = -53.0;
                self.y = 0.0;
                self.ext = Some(NeuriteExt {
                    k: 3.0,
                    gcc: 1.0,
                    gpc: 1.0,
                    vr: -56.0,
                    vt: -42.0,
                    vp: 40.0,
                    cap: 100.0,
                });
            }
            SpikeModel::MixedMode => {
                self.spike_model = spike_model;
                self.a = 0.02;
                self.b = 0.2;
                self.c = -55.0;
                self.d = 4.0;
                self.u = -14.0;
                self.v = -70.0;
                self.y = 0.0;
                self.ext = None;
            }
            SpikeModel::PhasicBursting => {
                self.spike_model = spike_model;
                self.a = 0.02;
                self.b = 0.25;
                self.c = -55.0;
                self.d = 0.05;
                self.u = -16.0;
                self.v = -64.0;
                self.y = 0.0;
                self.ext = None;
            }
            SpikeModel::PhasicSpiking => {
                self.spike_model = spike_model;
                self.a = 0.02;
                self.b = 0.25;
                self.c = -65.0;
                self.d = 6.0;
                self.u = -16.0;
                self.v = -64.0;
                self.y = 0.0;
                self.ext = None;
            }
            SpikeModel::ReboundBurst => {
                self.spike_model = spike_model;
                self.a = 0.03;
                self.b = 0.25;
                self.c = -52.0;
                self.d = 0.0;
                self.u = -16.0;
                self.v = -64.0;
                self.y = 0.0;
                self.ext = None;
            }
            SpikeModel::ReboundSpike => {
                self.spike_model = spike_model;
                self.a = 0.03;
                self.b = 0.25;
                self.c = -60.0;
                self.d = 4.0;
                self.u = -16.0;
                self.v = -64.0;
                self.y = 0.0;
                self.ext = None;
            }
            SpikeModel::RegularSpiking => {
                self.spike_model = spike_model;
                self.a = 0.02;
                self.b = 0.2;
                self.c = -65.0;
                self.d = 8.0;
                self.u = -12.6;
                self.v = -63.0;
                self.y = 0.0;
                self.ext = None;
            }
            SpikeModel::RegularSpikingPyramidalI => {
                self.spike_model = spike_model;
                self.a = 0.03;
                self.b = -2.0;
                self.c = -50.0;
                self.d = 100.0;
                self.u = 0.0;
                self.v = -60.0;
                self.y = 0.0;
                self.ext = Some(NeuriteExt {
                    k: 0.7,
                    gcc: 1.0,
                    gpc: 1.0,
                    vr: -60.0,
                    vt: -40.0,
                    vp: 35.0,
                    cap: 100.0,
                });
            }
            SpikeModel::RegularSpikingPyramidalII | SpikeModel::RegularSpikingSpinyStellate => {
                self.spike_model = spike_model;
                self.a = 0.01;
                self.b = 5.0;
                self.c = -60.0;
                self.d = 400.0;
                self.u = 0.0;
                self.v = -60.0;
                self.y = 0.0;
                self.ext = Some(NeuriteExt {
                    k: 3.0,
                    gcc: 3.0,
                    gpc: 5.0,
                    vr: -60.0,
                    vt: -50.0,
                    vp: 50.0,
                    cap: 100.0,
                });
            }
            SpikeModel::RegularSpikingPyramidalL2L3Dendrite => {
                self.spike_model = spike_model;
                self.a = 0.01;
                self.b = 5.0;
                self.c = -55.0;
                self.d = 400.0;
                self.u = 0.0;
                self.v = -60.0;
                self.y = 0.0;
                self.ext = Some(NeuriteExt {
                    k: 3.0,
                    gcc: 3.0,
                    gpc: 5.0,
                    vr: -60.0,
                    vt: -50.0,
                    vp: 30.0,
                    cap: 100.0,
                });
            }
            SpikeModel::RegularSpikingPyramidalL4Dendrite => {
                self.spike_model = spike_model;
                self.a = 0.01;
                self.b = 5.0;
                self.c = -50.0;
                self.d = 400.0;
                self.u = 0.0;
                self.v = -60.0;
                self.y = 0.0;
                self.ext = Some(NeuriteExt {
                    k: 3.0,
                    gcc: 3.0,
                    gpc: 5.0,
                    vr: -60.0,
                    vt: -50.0,
                    vp: 50.0,
                    cap: 100.0,
                });
            }
            SpikeModel::RegularSpikingPyramidalL5L6Dendrite | SpikeModel::RegularSpikingSpinyStellateDendrite => {
                self.spike_model = spike_model;
                self.a = 0.01;
                self.b = 5.0;
                self.c = -50.0;
                self.d = 400.0;
                self.u = 0.0;
                self.v = -60.0;
                self.y = 0.0;
                self.ext = Some(NeuriteExt {
                    k: 3.0,
                    gcc: 3.0,
                    gpc: 5.0,
                    vr: -60.0,
                    vt: -50.0,
                    vp: 30.0,
                    cap: 100.0,
                });
            }
            SpikeModel::ResonatorI => {
                self.spike_model = spike_model;
                self.a = 0.1;
                self.b = 0.26;
                self.c = -65.0;
                self.d = 2.0;
                self.u = -18.2;
                self.v = -70.0;
                self.y = 0.0;
                self.ext = None;
            }
            SpikeModel::ResonatorII => {
                self.spike_model = spike_model;
                self.a = 0.1;
                self.b = 0.26;
                self.c = -60.0;
                self.d = -1.0;
                self.u = -16.12;
                self.v = -62.0;
                self.y = 0.0;
                self.ext = None;
            }
            SpikeModel::ReticularThalamicNeuron => {
                self.spike_model = spike_model;
                self.a = 0.015;
                self.b = 10.0;
                self.c = -55.0;
                self.d = 50.0;
                self.u = 0.0;
                self.v = 0.0;
                self.y = 0.0;
                self.ext = Some(NeuriteExt {
                    k: 0.25,
                    gcc: 5.0,
                    gpc: 5.0,
                    vr: -65.0,
                    vt: -45.0,
                    vp: 0.0,
                    cap: 40.0,
                });
            }
            SpikeModel::SpikeFrequencyAdaptation => {
                self.spike_model = spike_model;
                self.a = 0.01;
                self.b = 0.2;
                self.c = -65.0;
                self.d = 8.0;
                self.u = -14.0;
                self.v = -70.0;
                self.y = 0.0;
                self.ext = None;
            }
            SpikeModel::SpikeLatency => {
                self.spike_model = spike_model;
                self.a = 0.02;
                self.b = 0.2;
                self.c = -65.0;
                self.d = 6.0;
                self.u = -14.0;
                self.v = -70.0;
                self.y = 0.0;
                self.ext = None;
            }
            SpikeModel::SpinyProjection => {
                self.spike_model = spike_model;
                self.a = 0.01;
                self.b = -20.0;
                self.c = -55.0;
                self.d = 150.0;
                self.u = 0.0;
                self.v = -80.0;
                self.y = 0.0;
                self.ext = Some(NeuriteExt {
                    k: 1.0,
                    gcc: 1.0,
                    gpc: 1.0,
                    vr: -80.0,
                    vt: -25.0,
                    vp: 40.0,
                    cap: 50.0,
                });
            }
            SpikeModel::SubthresholdOscillation => {
                self.spike_model = spike_model;
                self.a = 0.05;
                self.b = 0.26;
                self.c = -60.0;
                self.d = 0.0;
                self.u = -16.12;
                self.v = -62.0;
                self.y = 0.0;
                self.ext = None;
            }
            SpikeModel::ThalamicInterneuron => {
                self.spike_model = spike_model;
                self.a = 0.05;
                self.b = 7.0;
                self.c = -65.0;
                self.d = 50.0;
                self.u = 0.0;
                self.v = -60.0;
                self.y = 0.0;
                self.ext = Some(NeuriteExt {
                    k: 0.5,
                    gcc: 5.0,
                    gpc: 5.0,
                    vr: -60.0,
                    vt: -50.0,
                    vp: 20.0,
                    cap: 20.0,
                });
            }
            SpikeModel::Thalamocortical => {
                self.spike_model = spike_model;
                self.a = 0.1;
                self.b = 15.0;
                self.c = -60.0;
                self.d = 10.0;
                self.u = 0.0;
                self.v = -60.0;
                self.y = 0.0;
                self.ext = Some(NeuriteExt {
                    k: 1.6,
                    gcc: 2.0,
                    gpc: 2.0,
                    vr: -60.0,
                    vt: -50.0,
                    vp: 40.0,
                    cap: 200.0,
                });
            }
            SpikeModel::ThalamocorticalBursting => {
                self.spike_model = spike_model;
                self.a = 0.02;
                self.b = 0.25;
                self.c = -65.0;
                self.d = 0.05;
                self.u = -21.75;
                self.v = -87.0;
                self.y = 0.0;
                self.ext = None;
            }
            SpikeModel::ThalamocorticalSpiking => {
                self.spike_model = spike_model;
                self.a = 0.02;
                self.b = 0.25;
                self.c = -65.0;
                self.d = 0.05;
                self.u = -15.75;
                self.v = -63.0;
                self.y = 0.0;
                self.ext = None;
            }
            SpikeModel::ThresholdVariability => {
                self.spike_model = spike_model;
                self.a = 0.03;
                self.b = 0.25;
                self.c = -60.0;
                self.d = 4.0;
                self.u = -16.0;
                self.v = -64.0;
                self.y = 0.0;
                self.ext = None;
            }
            SpikeModel::TonicBursting => {
                self.spike_model = spike_model;
                self.a = 0.02;
                self.b = 0.2;
                self.c = -50.0;
                self.d = 2.0;
                self.u = -14.0;
                self.v = -70.0;
                self.y = 0.0;
                self.ext = None;
            }
            SpikeModel::TonicSpiking => {
                self.spike_model = spike_model;
                self.a = 0.02;
                self.b = 0.2;
                self.c = -65.0;
                self.d = 6.0;
                self.u = -14.0;
                self.v = -70.0;
                self.y = 0.0;
                self.ext = None;
            }
        }
    }

    /// Resets the neurites u, v, and y variables. If this is called on a neurite that has an extended spike model, an error message
    /// is returned.
    fn reset(&mut self) {
        match self.spike_model {
            SpikeModel::Accommodation => {
                self.u = -16.0;
                self.v = -65.0;
                self.y = 0.0;
            }
            SpikeModel::Bistability => {
                self.u = -15.86;
                self.v = -61.0;
                self.y = 0.0;
            }
            SpikeModel::ChatteringI | SpikeModel::DepolarizingAfterPotential | SpikeModel::FastSpiking | SpikeModel::MixedMode |
            SpikeModel::SpikeFrequencyAdaptation | SpikeModel::SpikeLatency | SpikeModel::TonicBursting | SpikeModel::TonicSpiking => {
                self.u = -14.0;
                self.v = -70.0;
                self.y = 0.0;
            }
            SpikeModel::ChatteringII | SpikeModel::EntorhinalStellate | SpikeModel::HippocampalCA1PyramidalHighThresholdBursting |
            SpikeModel::HippocampalCA1PyramidalLowThresholdBurstingI | SpikeModel::HippocampalCA1PyramidalLowThresholdBurstingII |
            SpikeModel::HippocampalCA1PyramidalNonBursting | SpikeModel::IntrinsicallyBurstingPyramidalDendriteII |
            SpikeModel::IntrinsicallyBurstingPyramidalSomaII | SpikeModel::RegularSpikingPyramidalI |
            SpikeModel::RegularSpikingPyramidalII | SpikeModel::RegularSpikingPyramidalL2L3Dendrite |
            SpikeModel::RegularSpikingPyramidalL4Dendrite | SpikeModel::RegularSpikingPyramidalL5L6Dendrite |
            SpikeModel::RegularSpikingSpinyStellate | SpikeModel::RegularSpikingSpinyStellateDendrite |
            SpikeModel::ThalamicInterneuron | SpikeModel::Thalamocortical => {
                self.u = 0.0;
                self.v = -60.0;
                self.y = 0.0;
            }
            SpikeModel::ClassI | SpikeModel::Integrator => {
                self.u = 6.0;
                self.v = -60.0;
                self.y = 0.0;
            }
            SpikeModel::ClassII => {
                self.u = -16.64;
                self.v = -64.0;
                self.y = 0.0;
            }
            SpikeModel::FastSpikingBasket => {
                self.u = 0.0;
                self.v = -55.0;
                self.y = 0.0;
            }
            SpikeModel::InhibitionInducedBursting | SpikeModel::InhibitionInducedSpiking => {
                self.u = 63.8;
                self.v = -63.8;
                self.y = 0.0;
            }
            SpikeModel::IntrinsicallyBurstingPyramidal => {
                self.u = 0.0;
                self.v = -75.0;
                self.y = 0.0;
            }
            SpikeModel::IntrinsicallyBurstingPyramidalDendriteI => {
                self.u = 0.0;
                self.v = -50.0;
                self.y = 0.0;
            }
            SpikeModel::IntrinsicallyBurstingPyramidalSomaI => {
                self.u = 0.0;
                self.v = -70.0;
                self.y = 0.0;
            }
            SpikeModel::LatentSpikingNonBasket | SpikeModel::LatentSpikingNonBasketDendrite  | 
            SpikeModel::LowThresholdSpikingNonBasket => {
                self.u = 0.0;
                self.v = -53.0;
                self.y = 0.0;
            }
            SpikeModel::LowThresholdSpiking => {
                self.u = -15.75;
                self.v = -63.0;
                self.y = 0.0;
            }
            SpikeModel::PhasicBursting | SpikeModel::PhasicSpiking | SpikeModel::ReboundBurst | SpikeModel::ReboundSpike |
            SpikeModel::ThresholdVariability => {
                self.u = -16.0;
                self.v = -64.0;
                self.y = 0.0;
            }
            SpikeModel::RegularSpiking => {
                self.u = -12.6;
                self.v = -63.0;
                self.y = 0.0;
            }
            SpikeModel::ResonatorI => {
                self.u = -18.2;
                self.v = -70.0;
                self.y = 0.0;
            }
            SpikeModel::ResonatorII | SpikeModel::SubthresholdOscillation => {
                self.u = -16.12;
                self.v = -62.0;
                self.y = 0.0;
            }
            SpikeModel::ReticularThalamicNeuron => {
                self.u = 0.0;
                self.v = 0.0;
                self.y = 0.0;
            }
            SpikeModel::SpinyProjection => {
                self.u = 0.0;
                self.v = -80.0;
                self.y = 0.0;
            }
            SpikeModel::ThalamocorticalBursting => {
                self.u = -15.75;
                self.v = -63.0;
                self.y = 0.0;
            }
            SpikeModel::ThalamocorticalSpiking => {
                self.u = -21.75;
                self.v = -87.0;
                self.y = 0.0;
            }
        }
    }

    /// Processes the neurite and returns the membrane potential and spike output values.
    fn process(&mut self, time: f64) -> (f64, f64) {
        match self.ext.as_deref() {
            Some(e) => {
                self.y = 0.0;

                match self.spike_model {
                    SpikeModel::IntrinsicallyBurstingPyramidalDendriteI => {
                        self.v += time * (e.k * (self.v - e.vr) * (self.v - e.vt) - e.vp * (self.vpc - self.v) - self.u + self.vcc) / e.cap;
                    }
                    SpikeModel::IntrinsicallyBurstingPyramidalSomaI => {
                        self.v += time * (e.k * (self.v - e.vr) * (self.v - e.vt) - e.vp * (self.vcc - self.v) - self.u + self.vpc) / e.cap;
                    }
                    _ => {
                        self.v += time * (e.k * (self.v - e.vr) * (self.v - e.vt) - self.u + self.vcc + self.vpc) / e.cap;
                    }
                }

                return match self.spike_model {
                    SpikeModel::EntorhinalStellate => {
                        self.u += time * self.a * (self.b * (self.v - e.vr) - self.u);

                        if self.v > e.vp {
                            self.v = self.c;
                            self.y = 1.0;

                            return (e.vp, self.y);
                        }

                        (self.v, self.y)
                    }
                    SpikeModel::FastSpikingBasket => {

                        if self.v < e.vr {
                            self.u += time * self.a * -self.u;
                        }
                        else {
                            self.u += time * self.a * (((0.025 * (self.v - e.vr)).pow(3.0)) - self.u);
                        }

                        if self.v > e.vp {
                            self.v = self.c;
                            self.y = 1.0;

                            return (e.vp, self.y);
                        }

                        (self.v, self.y)
                    }
                    SpikeModel::IntrinsicallyBurstingPyramidalDendriteI => {
                        self.u += time * self.a * (self.b * (self.v - e.vr) - self.u);

                        if self.v > e.vp {
                            self.v = self.c;
                            self.u += self.d;
                            self.y = 1.0;

                            return (e.vp, self.y);
                        }

                        (self.v, self.y)
                    }
                    SpikeModel::IntrinsicallyBurstingPyramidalSomaI => {
                        self.u += time * self.a * (self.b * (self.v - e.vr) - self.u);

                        if self.v > e.vp {
                            self.v = self.c;
                            self.u += self.d;
                            self.y = 1.0;

                            return (e.vp, self.y);
                        }

                        (self.v, self.y)
                    }
                    SpikeModel::LowThresholdSpikingNonBasket => {
                        self.u += time * self.a * (self.b * (self.v - e.vr) - self.u);

                        let peak = e.vp - 0.1 * self.u;

                        if self.v >= peak {
                            self.v = self.c + 0.04 * self.u;
                            self.u = (self.u + self.d).min(670.0);
                            self.y = 1.0;

                            return (peak, self.y);
                        }

                        (self.v, self.y)
                    }
                    SpikeModel::ReticularThalamicNeuron => {
                        if self.v > -65.0 {
                            self.b = 2.0;
                        }
                        else {
                            self.b = 10.0;
                        }

                        self.u += time * self.a * (self.b * (self.v - e.vr) - self.u);

                        if self.v > e.vp {
                            self.v = self.c;
                            self.u += self.d;
                            self.y = 1.0;

                            return (e.vp, self.y);
                        }

                        (self.v, self.y)
                    }
                    SpikeModel::ThalamicInterneuron => {
                        self.u += time * self.a * (self.b * (self.v - e.vr) - self.u);

                        let peak = e.vp - 0.08 * self.u;

                        if self.v >= peak {
                            self.v = self.c + 0.08 * self.u;
                            self.u = (self.u + self.d).min(530.0);
                            self.y = 1.0;

                            return (peak, self.y);
                        }

                        (self.v, self.y)
                    }
                    SpikeModel::Thalamocortical => {
                        if self.v > -65.0 {
                            self.b = 0.0;
                        }
                        else {
                            self.b = 15.0;
                        }

                        self.u += time * self.a * (self.b * (self.v - e.vr) - self.u);

                        let peak = e.vp + 0.1 * self.u;

                        if self.v >= peak {
                            self.v = self.c - 0.1 * self.u;
                            self.u += self.d;
                            self.y = 1.0;

                            return (peak, self.y);
                        }

                        (self.v, self.y)
                    }
                    _ => {
                        self.u += time * self.a * (self.b * (self.v - e.vr) - self.u);

                        if self.v > e.vp {
                            self.v = self.c;
                            self.u += self.d;
                            self.y = 1.0;

                            return (e.vp, self.y);
                        }

                        (self.v, self.y)
                    }
                }
            }
            None() => {
                self.y = 0.0;
                self.v += time * (0.04 * self.v * self.v + 5.0 * self.v + 140.0 - self.u + self.vcc + self.vcc);
                self.u += time * self.a * (self.b * self.v - self.u);

                if self.v >= 30.0 {
                    self.v = self.c;
                    self.u += self.d;
                    self.y = 1.0;

                    return (30.0, self.y);
                }

                (self.v, self.y)
            }
        }
    }

    /// Adds the specified child neurite. If the child cannot be added, this returns an error message specifying why.
    pub fn add_child(&mut self, child: Rc<RefCell<Neurite>>) -> Result<(), &'static str> {
        match self.neurite_type {
            // Add children to a soma neurite
            NeuriteType::Soma => {
                match child.borrow().neurite_type {
                    // Soma neurite cannot be a child of any neurite
                    NeuriteType::Soma => {
                        return Err("Cannot add a soma neurite as a child of a soma neurite.");
                    }
                    // Soma neurite can have any number of basal proximal neurite children
                    NeuriteType::BasalProximal => {
                        self.child.push(Rc::clone(&child));
                    }
                    // Soma neurite cannot have any basal distal neurite children
                    NeuriteType::BasalDistal => {
                        return Err("Cannot add a basal distal neurite as a child of a soma neurite.");
                    }
                    // Soma neurite can have one apical trunk neurite child
                    NeuriteType::ApicalTrunk => {
                        for i in 0..self.child.len() {
                            if self.child[i].borrow().neurite_type == NeuriteType::ApicalTrunk {
                                return Err("Cannot have more than one apical trunk neurite as a child of a soma neurite.");
                            }
                        }

                        self.child.push(Rc::clone(&child));
                    }
                    // Soma neurite cannot have any apical tuft neurite children
                    NeuriteType::ApicalTuft => {
                        return Err("Cannot add an apical tuft neurite as a child of a soma neurite.");
                    }
                    // Soma neurite can have one axon neurite child
                    NeuriteType::Axon => {
                        for i in 0..self.child.len() {
                            if self.child[i].borrow().neurite_type == NeuriteType::Axon {
                                return Err("Cannot have more than one axon neurite as a child of a soma neurite.");
                            }
                        }

                        self.child.push(Rc::clone(&child));
                    }
                }
            }
            // Add children to a basal proximal neurite
            NeuriteType::BasalProximal => {
                match child.borrow().neurite_type {
                    // Soma neurite cannot be a child of any neurite
                    NeuriteType::Soma => {
                        return Err("Cannot add a soma neurite as a child of a basal proximal neurite.");
                    }
                    // Basal proximal neurite can have any number of basal proximal neurite children
                    NeuriteType::BasalProximal => {
                        self.child.push(Rc::clone(&child));
                    }
                    // Basal proximal neurite can have any number of basal distal neurite children
                    NeuriteType::BasalDistal => {
                        self.child.push(Rc::clone(&child));
                    }
                    // Basal proximal neurite cannot have any apical trunk neurite children
                    NeuriteType::ApicalTrunk => {
                        return Err("Cannot add an apical trunk neurite as a child of a basal proximal neurite.");
                    }
                    // Basal proximal neurite cannot have any apical tuft neurite children
                    NeuriteType::ApicalTuft => {
                        return Err("Cannot add an apical tuft neurite as a child of a basal proximal neurite.");
                    }
                    // Basal proximal neurite cannot have any axon neurite children
                    NeuriteType::Axon => {
                        return Err("Cannot add an axon neurite as a child of a basal proximal neurite.");
                    }
                }
            }
            // Add children to a basal distal neurite
            NeuriteType::BasalDistal => {
                match child.borrow().neurite_type {
                    // Soma neurite cannot be a child of any neurite
                    NeuriteType::Soma => {
                        return Err("Cannot add a soma neurite as a child of a basal distal neurite.");
                    }
                    // Basal distal neurite cannot have any basal proximal neurite children
                    NeuriteType::BasalProximal => {
                        return Err("Cannot add a basal proximal neurite as a child of a basal distal neurite.");
                    }
                    // Basal distal neurite can have any number of basal distal neurite children
                    NeuriteType::BasalDistal => {
                        self.child.push(Rc::clone(&child));
                    }
                    // Basal distal neurite cannot have any apical trunk neurite children
                    NeuriteType::ApicalTrunk => {
                        return Err("Cannot add an apical trunk neurite as a child of a basal distal neurite.");
                    }
                    // Basal distal neurite cannot have any apical tuft neurite children
                    NeuriteType::ApicalTuft => {
                        return Err("Cannot add an apical tuft neurite as a child of a basal distal neurite.");
                    }
                    // Basal distal neurite cannot have any axon neurite children
                    NeuriteType::Axon => {
                        return Err("Cannot add an axon neurite as a child of a basal distal neurite.");
                    }
                }
            }
            // Add children to an apical trunk neurite
            NeuriteType::ApicalTrunk => {
                match child.borrow().neurite_type {
                    // Soma neurite cannot be a child of any neurite
                    NeuriteType::Soma => {
                        return Err("Cannot add a soma neurite as a child of an apical trunk neurite.");
                    }
                    // Apical trunk neurite cannot have any basal proximal neurite children
                    NeuriteType::BasalProximal => {
                        return Err("Cannot add a basal proximal neurite as a child of an apical trunk neurite.");
                    }
                    // Apical trunk neurite cannot have any basal distal neurite children
                    NeuriteType::BasalDistal => {
                        return Err("Cannot add a basal distal neurite as a child of an apical trunk neurite.");
                    }
                    // Apical trunk neurite can have any number of apical trunk neurite children
                    NeuriteType::ApicalTrunk => {
                        self.child.push(Rc::clone(&child));
                    }
                    // Apical trunk neurite can have any number of apical tuft neurite children
                    NeuriteType::ApicalTuft => {
                        self.child.push(Rc::clone(&child));
                    }
                    // Apical trunk neurite cannot have any axon neurite children
                    NeuriteType::Axon => {
                        return Err("Cannot add an axon neurite as a child of an apical trunk neurite.");
                    }
                }
            }
            // Add children to an apical tuft neurite
            NeuriteType::ApicalTuft => {
                match child.borrow().neurite_type {
                    // Soma neurite cannot be a child of any neurite
                    NeuriteType::Soma => {
                        return Err("Cannot add a soma neurite as a child of an apical tuft neurite.");
                    }
                    // Apical tuft neurite cannot have any basal proximal neurite children
                    NeuriteType::BasalProximal => {
                        return Err("Cannot add a basal proximal neurite as a child of an apical tuft neurite.");
                    }
                    // Apical tuft neurite cannot have any basal distal neurite children
                    NeuriteType::BasalDistal => {
                        return Err("Cannot add a basal distal neurite as a child of an apical tuft neurite.");
                    }
                    // Apical tuft neurite cannot have any apical trunk neurite children
                    NeuriteType::ApicalTrunk => {
                        return Err("Cannot add an apical trunk neurite as a child of an apical tuft neurite.");
                    }
                    // Apical tuft neurite can have any number of apical tuft neurite children
                    NeuriteType::ApicalTuft => {
                        self.child.push(Rc::clone(&child));
                    }
                    // Apical tuft neurite cannot have any axon neurite children
                    NeuriteType::Axon => {
                        return Err("Cannot add an axon neurite as a child of an apical tuft neurite.");
                    }
                }
            }
            // Add children to an axon neurite
            NeuriteType::Axon => {
                match child.borrow().neurite_type {
                    // Soma neurite cannot be a child of any neurite
                    NeuriteType::Soma => {
                        return Err("Cannot add a soma neurite as a child of an axon neurite.");
                    }
                    // Axon neurite cannot have any basal proximal neurite children
                    NeuriteType::BasalProximal => {
                        return Err("Cannot add a basal proximal neurite as a child of an axon neurite.");
                    }
                    // Axon neurite cannot have any basal distal neurite children
                    NeuriteType::BasalDistal => {
                        return Err("Cannot add a basal distal neurite as a child of an axon neurite.");
                    }
                    // Axon neurite cannot have any apical trunk neurite children
                    NeuriteType::ApicalTrunk => {
                        return Err("Cannot add an apical trunk neurite as a child of an axon neurite.");
                    }
                    // Axon neurite cannot have any apical tuft neurite children
                    NeuriteType::ApicalTuft => {
                        return Err("Cannot add an apical tuft neurite as a child of an axon neurite.");
                    }
                    // Axon neurite can have any number of axon neurite children
                    NeuriteType::Axon => {
                        self.child.push(Rc::clone(&child));
                    }
                }
            }
        }

        Ok(())
    }

    /// Removes the child at the specified index. All children of the removed child will become children of the
    /// removed child's parent. Returns false if index is out of bounds.
    pub fn remove_child(&mut self, index: usize) -> bool {
        if index >= self.child.len() {
            return false;
        }

        for i in 0..self.child[index].borrow().child.len() {
            self.child[index].clone().borrow_mut().child[i].borrow_mut().parent = Some(self.child[index].borrow().clone().parent.unwrap().clone());
            self.child.push(self.child[index].clone().borrow().child[i].clone());
        }

        self.child.remove(index);

        true
    }

    /// Removes the child at the specified index. All children of the removed child are removed as well. Returns
    /// false if index is out of bounds.
    pub fn prune_child(&mut self, index: usize) -> bool {
        if index >= self.child.len() {
            return false;
        }

        self.child.remove(index);

        true
    }

    /// Sets the parent. If the parent cannot be set, this returns an error message specifying why.
    pub fn set_parent(&mut self, parent: Rc<RefCell<Neurite>>) -> Result<(), &'static str> {
        match self.neurite_type {
            // Soma neurite cannot have a parent
            NeuriteType::Soma => {
                return Err("Soma neurite cannot have a parent neurite.");
            }
            // Basal proximal neurite parent must be a soma or basal proximal neurite
            NeuriteType::BasalProximal => {
                if parent.borrow().neurite_type != NeuriteType::Soma ||
                    parent.borrow().neurite_type != NeuriteType::BasalProximal {
                    return Err("Basal proximal neurite must have a soma or basal proximal neurite parent.");
                }

                self.parent = Some(parent);
            }
            // Basal distal neurite parent must be a basal proximal or basal distal neurite
            NeuriteType::BasalDistal => {
                if parent.borrow().neurite_type != NeuriteType::BasalDistal ||
                    parent.borrow().neurite_type != NeuriteType::BasalProximal {
                    return Err("Basal distal neurite must have a basal proximal or basal distal neurite parent.");
                }

                self.parent = Some(parent);
            }
            // Apical trunk neurite parent must be a soma or apical trunk neurite
            NeuriteType::ApicalTrunk => {
                if parent.borrow().neurite_type != NeuriteType::Soma ||
                    parent.borrow().neurite_type != NeuriteType::ApicalTrunk {
                    return Err("Apical trunk neurite must have a soma or apical trunk neurite parent.");
                }

                self.parent = Some(parent);
            }
            // Apical tuft neurite parent must be an apical trunk or apical tuft neurite
            NeuriteType::ApicalTuft => {
                if parent.borrow().neurite_type != NeuriteType::ApicalTrunk ||
                    parent.borrow().neurite_type != NeuriteType::ApicalTuft {
                    return Err("Apical tuft neurite must have an apical trunk or apical tuft neurite parent.");
                }

                self.parent = Some(parent);
            }
            // Axon neurite parent must be a soma or axon neurite
            NeuriteType::Axon => {
                if parent.borrow().neurite_type != NeuriteType::Soma ||
                    parent.borrow().neurite_type != NeuriteType::Axon {
                    return Err("Axon neurite must have a soma or axon neurite parent.");
                }

                self.parent = Some(parent);
            }
        }

        Ok(())
    }
}