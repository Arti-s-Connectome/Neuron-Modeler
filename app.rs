use egui::ImageButton;
use std::cell::RefCell;
use std::f64::consts::TAU;
use std::rc::Rc;
use eframe::App;
use egui::*;
use egui_extras::install_image_loaders;
use egui_plot::{Line, LineStyle, Plot, PlotItem, PlotPoint, PlotPoints};

/// Contains data for the style of the Neuron Modeler app.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct AppStyle {
    /// The color of a neuron's somatic neurite (dark mode).
    soma_dark_color: Color32,
    /// The color of a neuron's proximal basal dendritic neurites (dark mode).
    proximal_dark_color: Color32,
    /// The color of a neuron's distal basal dendritic neurites (dark mode).
    distal_dark_color: Color32,
    /// The color of a neuron's apical trunk dendritic neurites (dark mode).
    trunk_dark_color: Color32,
    /// The color of a neuron's apical tuft dendritic neurites (dark mode).
    tuft_dark_color: Color32,
    /// The color of a neuron's axonal neurites (dark mode).
    axon_dark_color: Color32,
    /// The color of a neuron's excitatory synapses (dark mode).
    excitatory_dark_color: Color32,
    /// The color of a neuron's fast inhibitory synapses (dark mode).
    fast_inhibitory_dark_color: Color32,
    /// The color of a neuron's slow inhibitory synapses (dark mode).
    slow_inhibitory_dark_color: Color32,
    /// The color of a neuron's synaptic modulatory synapses (dark mode).
    synaptic_modulatory_dark_color: Color32,
    /// The color of a neuron's neural modulatory synapses (dark mode).
    neural_modulatory_dark_color: Color32,
    /// The color of line connectors (dark mode).
    line_dark_color: Color32,
    /// The color of a neuron's somatic neurite (light mode).
    soma_light_color: Color32,
    /// The color of a neuron's proximal basal dendritic neurites (light mode).
    proximal_light_color: Color32,
    /// The color of a neuron's distal basal dendritic neurites (light mode).
    distal_light_color: Color32,
    /// The color of a neuron's apical trunk dendritic neurites (light mode).
    trunk_light_color: Color32,
    /// The color of a neuron's apical tuft dendritic neurites (light mode).
    tuft_light_color: Color32,
    /// The color of a neuron's axonal neurites (light mode).
    axon_light_color: Color32,
    /// The color of a neuron's excitatory synapses (light mode).
    excitatory_light_color: Color32,
    /// The color of a neuron's fast inhibitory synapses (light mode).
    fast_inhibitory_light_color: Color32,
    /// The color of a neuron's slow inhibitory synapses (light mode).
    slow_inhibitory_light_color: Color32,
    /// The color of a neuron's synaptic modulatory synapses (light mode).
    synaptic_modulatory_light_color: Color32,
    /// The color of a neuron's neural modulatory synapses (light mode).
    neural_modulatory_light_color: Color32,
    /// The color of line connectors (light mode).
    line_light_color: Color32,
    /// Set if the grid is on.
    grid_on: bool,
}

// Default for AppStyle
impl Default for AppStyle {
    fn default() -> Self {
        Self {
            soma_dark_color: Color32::from_rgb(255, 255, 102),
            proximal_dark_color: Color32::from_rgb(0, 255, 127),
            distal_dark_color: Color32::from_rgb(135,206,230),
            trunk_dark_color: Color32::from_rgb(221, 160, 221),
            tuft_dark_color: Color32::from_rgb(255, 0, 255),
            axon_dark_color: Color32::from_rgb(255, 91, 71),
            excitatory_dark_color: Color32::from_rgb(124, 252, 0),
            fast_inhibitory_dark_color: Color32::from_rgb(255, 0, 0),
            slow_inhibitory_dark_color: Color32::from_rgb(255, 165, 0),
            synaptic_modulatory_dark_color: Color32::from_rgb(65, 105, 225),
            neural_modulatory_dark_color: Color32::from_rgb(128, 43, 226),
            line_dark_color: Color32::WHITE,
            soma_light_color: Color32::from_rgb(153, 153, 0),
            proximal_light_color: Color32::from_rgb(60, 179, 113),
            distal_light_color: Color32::from_rgb(100, 149, 237),
            trunk_light_color: Color32::from_rgb(186, 85, 211),
            tuft_light_color: Color32::from_rgb(128, 0, 128),
            axon_light_color: Color32::from_rgb(220, 20, 60),
            excitatory_light_color: Color32::from_rgb(34, 139, 34),
            fast_inhibitory_light_color: Color32::from_rgb(139, 0, 0),
            slow_inhibitory_light_color: Color32::from_rgb(255, 140, 0),
            synaptic_modulatory_light_color: Color32::from_rgb(0, 0, 128),
            neural_modulatory_light_color: Color32::from_rgb(75, 0, 130),
            line_light_color: Color32::DARK_BLUE,
            grid_on: true
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Contains data for the Neuron Modeler app.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct NeuronModelerApp {
    /// Set if dark mode is on.
    dark_mode: bool,
    /// Set if app window is maximized.
    maximized: bool,
    /// Set if the main menu is open.
    #[serde(skip)]
    main_menu_open: bool,
    /// Set if settings is open.
    #[serde(skip)]
    settings_open: bool,
    /// Set if the style settings window is open.
    #[serde(skip)]
    style_settings_open: bool,
    /// The app's style data.
    app_style: AppStyle
}

// Default function for NeuronModelerApp
impl Default for NeuronModelerApp {
    fn default() -> Self {
        Self {
            dark_mode: false,
            maximized: false,
            main_menu_open: false,
            settings_open: false,
            style_settings_open: false,
            app_style: AppStyle::default()
        }
    }
}

// NeuronModelerApp functions
impl NeuronModelerApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        install_image_loaders(&cc.egui_ctx);

        // Load previous app state (if any).
        if let Some(storage) = cc.storage {
            let app: NeuronModelerApp = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();

            if app.dark_mode {
                cc.egui_ctx.set_visuals(Visuals::dark());
            }
            else {
                cc.egui_ctx.set_visuals(Visuals::light());
            }

            cc.egui_ctx.send_viewport_cmd(ViewportCommand::Maximized(app.maximized));

            return app;
        }

        Default::default()
    }

    /// Create the window frame.
    fn window_frame(&mut self, ctx: &Context, title: &str, add_contents: impl FnOnce(&mut Ui)) {
        let panel_frame = Frame {
            fill: ctx.style().visuals.window_fill(),
            rounding: 0.0.into(),
            stroke: ctx.style().visuals.widgets.noninteractive.fg_stroke,
            outer_margin: 0.5.into(), // so the stroke is within the bounds
            ..Default::default()
        };

        CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
            let app_rect = ui.max_rect();

            let title_bar_height = 32.0;
            let title_bar_rect = {
                let mut rect = app_rect;
                rect.max.y = rect.min.y + title_bar_height;
                rect
            };
            self.title_bar_ui(ui, title_bar_rect, title);

            let content_rect = {
                let mut rect = app_rect;
                rect.min.y = title_bar_rect.max.y;
                rect
            }
                .shrink(4.0);
            let mut content_ui = ui.child_ui(content_rect, *ui.layout(), None);
            add_contents(&mut content_ui);
        });

        // Show style settings window if it is open
        if self.style_settings_open {
            self.style_settingse(ctx);
        }
    }

    /// Create the title bar.
    fn title_bar_ui(&mut self, ui: &mut Ui, title_bar_rect: Rect, title: &str) {
        let painter = ui.painter();

        let title_bar_response = ui.interact(
            title_bar_rect,
            Id::new("title_bar"),
            Sense::click_and_drag(),
        );

        // Paint the line under the title
        painter.line_segment(
            [
                title_bar_rect.left_bottom() + vec2(1.0, 0.0),
                title_bar_rect.right_bottom() + vec2(-1.0, 0.0),
            ],
            ui.visuals().widgets.noninteractive.bg_stroke,
        );

        // Change window maximization when double-clicking the title bar
        if title_bar_response.double_clicked() {
            self.maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
            ui.ctx().send_viewport_cmd(ViewportCommand::Maximized(!self.maximized));
        }

        // Drag window when dragging on title bar
        if title_bar_response.drag_started_by(PointerButton::Primary) {
            ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
        }

        // Add close/maximize/minimize/settings buttons
        ui.allocate_ui_at_rect(title_bar_rect, |ui| {
            let left_rect = Rect{ min: Pos2 { x: 0.0, y: 0.0 },
                max: Pos2 { x: title_bar_rect.max.x / 2.0, y: title_bar_rect.max.y } };
            let right_rect = Rect{ min: Pos2 { x: title_bar_rect.max.x / 2.0, y: 0.0 },
                max: Pos2 { x: title_bar_rect.max.x, y: title_bar_rect.max.y } };

            ui.allocate_ui_at_rect(left_rect, |ui| {
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    ui.add(Image::new(include_image!("../assets/icons/icons8-neuron-64.png").clone()))
                        .on_hover_text(title);
                    self.main_menu(ui);
                });
            });

            ui.allocate_ui_at_rect(right_rect, |ui| {
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.visuals_mut().button_frame = false;
                    self.close_maximize_minimize(ui);
                    ui.add_space(32.0);
                    self.settings(ui);
                });
            });
        });
    }

    /// Show main menu button.
    fn main_menu(&mut self, ui: &mut Ui) {
        let mut next_pos = ui.next_widget_position();
        let rect = Rect::from_min_max(Pos2{x: next_pos.x + 2.0, y: next_pos.y - 14.0},
                                      Pos2{x: next_pos.x + 30.0, y: next_pos.y + 14.0});

        // Main menu button
        ui.allocate_ui_at_rect(rect, |ui| {
            let main_menu_button = ImageButton::new(include_image!("../assets/icons/icons8-menu-80.png").clone())
                .rounding(Rounding::same(5.0));

            let main_menu_response = Rc::new(RefCell::new(main_menu_button.clone().ui(ui)
                .on_hover_text("Main Menu")));

            if main_menu_response.borrow().clone().clicked() {
                self.main_menu_open = !self.main_menu_open;
            }
        });

        // Display main menu bar, if it is open
        if self.main_menu_open {
            ui.horizontal_centered(|ui| {
                menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Exit").clicked() {
                            ui.ctx().send_viewport_cmd(ViewportCommand::Close);
                        }
                    });
                })
            });
        }
    }

    /// Show settings button.
    fn settings(&mut self, ui: &mut Ui) {
        let next_pos = ui.next_widget_position();
        let rect = Rect::from_min_max(Pos2{x: next_pos.x - 30.0, y: next_pos.y - 14.0},
                                      Pos2{x: next_pos.x - 2.0, y: next_pos.y + 14.0});

        // Settings button
        ui.allocate_ui_at_rect(rect, |ui| {
            let settings_button = ImageButton::new(include_image!("../assets/icons/icons8-settings-80.png").clone())
                .rounding(Rounding::same(5.0));

            let settings_response = Rc::new(RefCell::new(settings_button.clone().ui(ui)
                .on_hover_text("Settings")));

            if settings_response.borrow().clone().clicked() {
                self.settings_open = !self.settings_open;
            }
        });

        // Display settings options, if it is open
        if self.settings_open {
            ui.add_space(2.0);

            ui.allocate_ui(Vec2::new(64.0, 28.0), |ui| {
                // Light/dark mode button
                let mode_button;
                let mode_response;

                if self.dark_mode {
                    mode_button = ImageButton::new(include_image!("../assets/icons/icons8-sun-80.png").clone())
                        .rounding(Rounding::same(5.0));

                    mode_response = Rc::new(RefCell::new(mode_button.clone().ui(ui)
                        .on_hover_text("Light Mode")));

                    if mode_response.borrow().clone().clicked() {
                        ui.ctx().set_visuals(Visuals::light());
                        self.dark_mode = false;
                        self.settings_open = false;
                    }
                }
                else {
                    mode_button = ImageButton::new(include_image!("../assets/icons/icons8-moon-and-stars-80.png").clone())
                        .rounding(Rounding::same(5.0));

                    mode_response = Rc::new(RefCell::new(mode_button.clone().ui(ui)
                        .on_hover_text("Dark Mode")));

                    if mode_response.borrow().clone().clicked() {
                        ui.ctx().set_visuals(Visuals::dark());
                        self.dark_mode = true;
                        self.settings_open = false;
                    }
                }

                ui.add_space(2.0);

                // Color wheel button
                let color_wheel_button = ImageButton::new(include_image!("../assets/icons/icons8-rgb-color-wheel-80.png").clone())
                    .rounding(Rounding::same(5.0));

                let color_wheel_response = Rc::new(RefCell::new(color_wheel_button.clone().ui(ui)
                    .on_hover_text("Color Style")));

                if color_wheel_response.borrow().clone().clicked() {
                    self.style_settings_open = true;
                    self.settings_open = false;
                }
            });
        }
    }

    /// Show close/maximize/minimize buttons.
    fn close_maximize_minimize(&mut self, ui: &mut Ui) {
        let mut next_pos = ui.next_widget_position();
        let mut rect = Rect::from_min_max(Pos2{x: next_pos.x - 31.0, y: next_pos.y - 15.0},
                                      Pos2{x: next_pos.x - 1.0, y: next_pos.y + 15.0});

        // Close button
        ui.allocate_ui_at_rect(rect, |ui| {
            let close_button = ImageButton::new(include_image!("../assets/icons/icons8-close-80.png").clone());

            let close_response = Rc::new(RefCell::new(close_button.clone().ui(ui)
                .on_hover_text("Close")));

            if close_response.borrow().clone().clicked() {
                ui.ctx().send_viewport_cmd(ViewportCommand::Close);
            }
        });

        // Dark mode
        if self.dark_mode {
            next_pos = ui.next_widget_position();
            rect = Rect::from_min_max(Pos2{x: next_pos.x - 31.0, y: next_pos.y - 15.0},
                                      Pos2{x: next_pos.x - 1.0, y: next_pos.y + 15.0});

            // Maximize button
            ui.allocate_ui_at_rect(rect, |ui| {
                let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));

                if is_maximized {
                    let maximized_button =
                        ImageButton::new(include_image!("../assets/icons/icons8-restore-window-50-dark.png").clone());

                    let maximized_response = Rc::new(RefCell::new(maximized_button.clone().ui(ui)
                        .on_hover_text("Restore")));

                    if maximized_response.borrow().clone().clicked() {
                        ui.ctx().send_viewport_cmd(ViewportCommand::Maximized(false));
                        self.maximized = false;
                    }
                } else {
                    let maximized_button =
                        ImageButton::new(include_image!("../assets/icons/icons8-maximize-50-dark.png").clone());

                    let maximized_response = Rc::new(RefCell::new(maximized_button.clone().ui(ui)
                        .on_hover_text("Maximize")));

                    if maximized_response.borrow().clone().clicked() {
                        ui.ctx().send_viewport_cmd(ViewportCommand::Maximized(true));
                        self.maximized = true;
                    }
                }
            });

            next_pos = ui.next_widget_position();
            rect = Rect::from_min_max(Pos2{x: next_pos.x - 31.0, y: next_pos.y - 15.0},
                                      Pos2{x: next_pos.x - 1.0, y: next_pos.y + 15.0});

            // Minimized button
            ui.allocate_ui_at_rect(rect, |ui| {
                let minimized_button =
                    ImageButton::new(include_image!("../assets/icons/icons8-minimize-96-dark.png").clone());

                let minimized_response = Rc::new(RefCell::new(minimized_button.clone().ui(ui)
                    .on_hover_text("Minimize")));

                if minimized_response.borrow().clone().clicked() {
                    ui.ctx().send_viewport_cmd(ViewportCommand::Minimized(true));
                }
            });
        }
        // Light mode
        else {
            next_pos = ui.next_widget_position();
            rect = Rect::from_min_max(Pos2{x: next_pos.x - 31.0, y: next_pos.y - 15.0},
                                      Pos2{x: next_pos.x - 1.0, y: next_pos.y + 15.0});

            // Maximize button
            ui.allocate_ui_at_rect(rect, |ui| {
                let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));

                if is_maximized {
                    let maximized_button =
                        ImageButton::new(include_image!("../assets/icons/icons8-restore-window-50-light.png").clone());

                    let maximized_response = Rc::new(RefCell::new(maximized_button.clone().ui(ui)
                        .on_hover_text("Restore")));

                    if maximized_response.borrow().clone().clicked() {
                        ui.ctx().send_viewport_cmd(ViewportCommand::Maximized(false));
                        self.maximized = false;
                    }
                } else {
                    let maximized_button =
                        ImageButton::new(include_image!("../assets/icons/icons8-maximize-50-light.png").clone());

                    let maximized_response = Rc::new(RefCell::new(maximized_button.clone().ui(ui)
                        .on_hover_text("Maximize")));

                    if maximized_response.borrow().clone().clicked() {
                        ui.ctx().send_viewport_cmd(ViewportCommand::Maximized(true));
                        self.maximized = true;
                    }
                }
            });

            next_pos = ui.next_widget_position();
            rect = Rect::from_min_max(Pos2{x: next_pos.x - 31.0, y: next_pos.y - 15.0},
                                      Pos2{x: next_pos.x - 1.0, y: next_pos.y + 15.0});

            // Minimized button
            ui.allocate_ui_at_rect(rect, |ui| {
                let minimized_button =
                    ImageButton::new(include_image!("../assets/icons/icons8-minimize-96-light.png").clone());

                let minimized_response = Rc::new(RefCell::new(minimized_button.clone().ui(ui)
                    .on_hover_text("Minimize")));

                if minimized_response.borrow().clone().clicked() {
                    ui.ctx().send_viewport_cmd(ViewportCommand::Minimized(true));
                }
            });
        }
    }

    /// Show the style settings window.
    fn style_settingse(&mut self, ctx: &Context) {
        Window::new("Style Settings")
            .open(&mut self.style_settings_open)
            .anchor(Align2::CENTER_CENTER, Vec2::new(0.0, 0.0))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        // Reset defaults button
                        if ui.button("Reset defaults").clicked() {
                            self.app_style = AppStyle::default();
                        }
                        ui.add_space(5.0);

                        // Light/dark mode button
                        global_dark_light_mode_buttons(ui);
                        ui.add_space(5.0);

                        self.dark_mode = ui.ctx().style().visuals.dark_mode;

                        // Show grid checkbox
                        ui.checkbox(&mut self.app_style.grid_on, "Show grid");
                        ui.add_space(5.0);

                        // Dark mode color settings
                        ui.collapsing("Dark Mode Colors", |ui| {
                            ui.horizontal(|ui| {
                                ui.add_space(32.0);
                                ui.color_edit_button_srgba(&mut self.app_style.soma_dark_color);
                                ui.label("Soma");
                            });
                            ui.end_row();

                            ui.horizontal(|ui| {
                                ui.add_space(32.0);
                                ui.color_edit_button_srgba(&mut self.app_style.proximal_dark_color);
                                ui.label("Proximal Basal Dendrite");
                            });
                            ui.end_row();

                            ui.horizontal(|ui| {
                                ui.add_space(32.0);
                                ui.color_edit_button_srgba(&mut self.app_style.distal_dark_color);
                                ui.label("Distal Basal Dendrite");
                            });
                            ui.end_row();

                            ui.horizontal(|ui| {
                                ui.add_space(32.0);
                                ui.color_edit_button_srgba(&mut self.app_style.trunk_dark_color);
                                ui.label("Apical Trunk Dendrite");
                            });
                            ui.end_row();

                            ui.horizontal(|ui| {
                                ui.add_space(32.0);
                                ui.color_edit_button_srgba(&mut self.app_style.tuft_dark_color);
                                ui.label("Apical Tuft Dendrite");
                            });
                            ui.end_row();

                            ui.horizontal(|ui| {
                                ui.add_space(32.0);
                                ui.color_edit_button_srgba(&mut self.app_style.axon_dark_color);
                                ui.label("Axon");
                            });
                            ui.end_row();

                            ui.horizontal(|ui| {
                                ui.add_space(32.0);
                                ui.color_edit_button_srgba(&mut self.app_style.excitatory_dark_color);
                                ui.label("Excitatory Synapse");
                            });
                            ui.end_row();

                            ui.horizontal(|ui| {
                                ui.add_space(32.0);
                                ui.color_edit_button_srgba(&mut self.app_style.fast_inhibitory_dark_color);
                                ui.label("Fast Inhibitory Synapse");
                            });
                            ui.end_row();

                            ui.horizontal(|ui| {
                                ui.add_space(32.0);
                                ui.color_edit_button_srgba(&mut self.app_style.slow_inhibitory_dark_color);
                                ui.label("Slow Inhibitory Synapse");
                            });
                            ui.end_row();

                            ui.horizontal(|ui| {
                                ui.add_space(32.0);
                                ui.color_edit_button_srgba(&mut self.app_style.synaptic_modulatory_dark_color);
                                ui.label("Synaptic Modulatory Synapse");
                            });
                            ui.end_row();

                            ui.horizontal(|ui| {
                                ui.add_space(32.0);
                                ui.color_edit_button_srgba(&mut self.app_style.neural_modulatory_dark_color);
                                ui.label("Neural Modulatory Synapse");
                            });
                            ui.end_row();

                            ui.horizontal(|ui| {
                                ui.add_space(32.0);
                                ui.color_edit_button_srgba(&mut self.app_style.line_dark_color);
                                ui.label("Line Connectors");
                            });
                            ui.end_row();
                        });

                        // Light mode color settings
                        ui.collapsing("Light Mode Colors", |ui| {
                            ui.horizontal(|ui| {
                                ui.add_space(32.0);
                                ui.color_edit_button_srgba(&mut self.app_style.soma_light_color);
                                ui.label("Soma");
                            });
                            ui.end_row();

                            ui.horizontal(|ui| {
                                ui.add_space(32.0);
                                ui.color_edit_button_srgba(&mut self.app_style.proximal_light_color);
                                ui.label("Proximal Basal Dendrite");
                            });
                            ui.end_row();

                            ui.horizontal(|ui| {
                                ui.add_space(32.0);
                                ui.color_edit_button_srgba(&mut self.app_style.distal_light_color);
                                ui.label("Distal Basal Dendrite");
                            });
                            ui.end_row();

                            ui.horizontal(|ui| {
                                ui.add_space(32.0);
                                ui.color_edit_button_srgba(&mut self.app_style.trunk_light_color);
                                ui.label("Apical Trunk Dendrite");
                            });
                            ui.end_row();

                            ui.horizontal(|ui| {
                                ui.add_space(32.0);
                                ui.color_edit_button_srgba(&mut self.app_style.tuft_light_color);
                                ui.label("Apical Tuft Dendrite");
                            });
                            ui.end_row();

                            ui.horizontal(|ui| {
                                ui.add_space(32.0);
                                ui.color_edit_button_srgba(&mut self.app_style.axon_light_color);
                                ui.label("Axon");
                            });
                            ui.end_row();

                            ui.horizontal(|ui| {
                                ui.add_space(32.0);
                                ui.color_edit_button_srgba(&mut self.app_style.excitatory_light_color);
                                ui.label("Excitatory Synapse");
                            });
                            ui.end_row();

                            ui.horizontal(|ui| {
                                ui.add_space(32.0);
                                ui.color_edit_button_srgba(&mut self.app_style.fast_inhibitory_light_color);
                                ui.label("Fast Inhibitory Synapse");
                            });
                            ui.end_row();

                            ui.horizontal(|ui| {
                                ui.add_space(32.0);
                                ui.color_edit_button_srgba(&mut self.app_style.slow_inhibitory_light_color);
                                ui.label("Slow Inhibitory Synapse");
                            });
                            ui.end_row();

                            ui.horizontal(|ui| {
                                ui.add_space(32.0);
                                ui.color_edit_button_srgba(&mut self.app_style.synaptic_modulatory_light_color);
                                ui.label("Synaptic Modulatory Synapse");
                            });
                            ui.end_row();

                            ui.horizontal(|ui| {
                                ui.add_space(32.0);
                                ui.color_edit_button_srgba(&mut self.app_style.neural_modulatory_light_color);
                                ui.label("Neural Modulatory Synapse");
                            });
                            ui.end_row();

                            ui.horizontal(|ui| {
                                ui.add_space(32.0);
                                ui.color_edit_button_srgba(&mut self.app_style.line_light_color);
                                ui.label("Line Connectors");
                            });
                            ui.end_row();
                        });
                    });

                    // Neuron plot region
                    ui.vertical(|ui| {
                        ui.horizontal_top(|ui| {
                            let mut plot = Plot::new("neuron_demo")
                                .data_aspect(1.0)
                                .height(600.0)
                                .width(600.0)
                                .show_grid(self.app_style.grid_on)
                                .show_axes(Vec2b::new(false, false))
                                .show_x(false)
                                .show_y(false);

                            let soma;
                            let proximal;
                            let distal;
                            let trunk;
                            let tuft;
                            let axon;
                            let synapse;

                            if self.dark_mode {
                                soma = Shape::triangle(Pos2::new(0.0, 0.0), 1.0, self.app_style.soma_dark_color);
                                proximal = vec![
                                    Shape::circle(Pos2::new(-2.0, -2.0), 0.25, self.app_style.proximal_dark_color),
                                    Shape::circle(Pos2::new(2.0, -2.0), 0.25, self.app_style.proximal_dark_color),
                                    Shape::circle(Pos2::new(-4.0, -2.0), 0.25, self.app_style.proximal_dark_color),
                                    Shape::circle(Pos2::new(4.0, -2.0), 0.25, self.app_style.proximal_dark_color),
                                    Shape::circle(Pos2::new(-3.0, -4.0), 0.25, self.app_style.proximal_dark_color),
                                    Shape::circle(Pos2::new(3.0, -4.0), 0.25, self.app_style.proximal_dark_color)
                                ];
                                distal = vec![
                                    Shape::circle(Pos2::new(-5.0, 0.0), 0.25, self.app_style.distal_dark_color),
                                    Shape::circle(Pos2::new(5.0, 0.0), 0.25, self.app_style.distal_dark_color),
                                    Shape::circle(Pos2::new(-5.0, -4.0), 0.25, self.app_style.distal_dark_color),
                                    Shape::circle(Pos2::new(5.0, -4.0), 0.25, self.app_style.distal_dark_color),
                                    Shape::circle(Pos2::new(-6.0, -2.0), 0.25, self.app_style.distal_dark_color),
                                    Shape::circle(Pos2::new(6.0, -2.0), 0.25, self.app_style.distal_dark_color),
                                    Shape::circle(Pos2::new(-4.0, -6.0), 0.25, self.app_style.distal_dark_color),
                                    Shape::circle(Pos2::new(4.0, -6.0), 0.25, self.app_style.distal_dark_color)
                                ];
                                trunk = vec![
                                    Shape::circle(Pos2::new(0.0, 3.0), 0.25, self.app_style.trunk_dark_color),
                                    Shape::circle(Pos2::new(0.0, 5.0), 0.25, self.app_style.trunk_dark_color),
                                    Shape::circle(Pos2::new(0.0, 7.0), 0.25, self.app_style.trunk_dark_color),
                                    Shape::circle(Pos2::new(0.0, 9.0), 0.25, self.app_style.trunk_dark_color)
                                ];
                                tuft = vec![
                                    Shape::circle(Pos2::new(-1.0, 11.0), 0.25, self.app_style.tuft_dark_color),
                                    Shape::circle(Pos2::new(1.0, 11.0), 0.25, self.app_style.tuft_dark_color),
                                    Shape::circle(Pos2::new(-1.0, 13.0), 0.25, self.app_style.tuft_dark_color),
                                    Shape::circle(Pos2::new(1.0, 13.0), 0.25, self.app_style.tuft_dark_color),
                                    Shape::circle(Pos2::new(-3.0, 12.0), 0.25, self.app_style.tuft_dark_color),
                                    Shape::circle(Pos2::new(3.0, 12.0), 0.25, self.app_style.tuft_dark_color),
                                    Shape::circle(Pos2::new(-3.0, 14.0), 0.25, self.app_style.tuft_dark_color),
                                    Shape::circle(Pos2::new(3.0, 14.0), 0.25, self.app_style.tuft_dark_color),
                                    Shape::circle(Pos2::new(-4.0, 14.0), 0.25, self.app_style.tuft_dark_color),
                                    Shape::circle(Pos2::new(4.0, 14.0), 0.25, self.app_style.tuft_dark_color),
                                    Shape::circle(Pos2::new(-1.0, 15.0), 0.25, self.app_style.tuft_dark_color),
                                    Shape::circle(Pos2::new(1.0, 15.0), 0.25, self.app_style.tuft_dark_color),
                                    Shape::circle(Pos2::new(-5.0, 12.0), 0.25, self.app_style.tuft_dark_color),
                                    Shape::circle(Pos2::new(5.0, 12.0), 0.25, self.app_style.tuft_dark_color),
                                ];
                                axon = vec![
                                    Shape::circle(Pos2::new(0.0, -3.0), 0.25, self.app_style.axon_dark_color),
                                    Shape::circle(Pos2::new(0.0, -5.0), 0.25, self.app_style.axon_dark_color),
                                    Shape::circle(Pos2::new(0.0, -7.0), 0.25, self.app_style.axon_dark_color),
                                    Shape::circle(Pos2::new(0.0, -9.0), 0.25, self.app_style.axon_dark_color),
                                    Shape::circle(Pos2::new(0.0, -11.0), 0.25, self.app_style.axon_dark_color),
                                    Shape::circle(Pos2::new(0.0, -13.0), 0.25, self.app_style.axon_dark_color),
                                    Shape::circle(Pos2::new(-1.0, -15.0), 0.25, self.app_style.axon_dark_color),
                                    Shape::circle(Pos2::new(1.0, -15.0), 0.25, self.app_style.axon_dark_color),
                                    Shape::circle(Pos2::new(-1.0, -17.0), 0.25, self.app_style.axon_dark_color),
                                    Shape::circle(Pos2::new(1.0, -17.0), 0.25, self.app_style.axon_dark_color),
                                    Shape::circle(Pos2::new(-3.0, -16.0), 0.25, self.app_style.axon_dark_color),
                                    Shape::circle(Pos2::new(3.0, -16.0), 0.25, self.app_style.axon_dark_color),
                                    Shape::circle(Pos2::new(5.0, -16.0), 0.25, self.app_style.axon_dark_color)
                                ];
                                synapse = vec![
                                    Shape::square(Pos2::new(-3.0, -18.0), 0.1, self.app_style.excitatory_dark_color),
                                    Shape::square(Pos2::new(-1.0, -19.0), 0.1, self.app_style.fast_inhibitory_dark_color),
                                    Shape::square(Pos2::new(5.0, -18.0), 0.1, self.app_style.slow_inhibitory_dark_color),
                                    Shape::square(Pos2::new(1.0, -19.0), 0.1, self.app_style.synaptic_modulatory_dark_color),
                                    Shape::square(Pos2::new(3.0, -18.0), 0.1, self.app_style.neural_modulatory_dark_color),
                                ];
                            }
                            else {
                                soma = Shape::triangle(Pos2::new(0.0, 0.0), 1.0, self.app_style.soma_light_color);
                                proximal = vec![
                                    Shape::circle(Pos2::new(-2.0, -2.0), 0.25, self.app_style.proximal_light_color),
                                    Shape::circle(Pos2::new(2.0, -2.0), 0.25, self.app_style.proximal_light_color),
                                    Shape::circle(Pos2::new(-4.0, -2.0), 0.25, self.app_style.proximal_light_color),
                                    Shape::circle(Pos2::new(4.0, -2.0), 0.25, self.app_style.proximal_light_color),
                                    Shape::circle(Pos2::new(-3.0, -4.0), 0.25, self.app_style.proximal_light_color),
                                    Shape::circle(Pos2::new(3.0, -4.0), 0.25, self.app_style.proximal_light_color)
                                ];
                                distal = vec![
                                    Shape::circle(Pos2::new(-5.0, 0.0), 0.25, self.app_style.distal_light_color),
                                    Shape::circle(Pos2::new(5.0, 0.0), 0.25, self.app_style.distal_light_color),
                                    Shape::circle(Pos2::new(-5.0, -4.0), 0.25, self.app_style.distal_light_color),
                                    Shape::circle(Pos2::new(5.0, -4.0), 0.25, self.app_style.distal_light_color),
                                    Shape::circle(Pos2::new(-6.0, -2.0), 0.25, self.app_style.distal_light_color),
                                    Shape::circle(Pos2::new(6.0, -2.0), 0.25, self.app_style.distal_light_color),
                                    Shape::circle(Pos2::new(-4.0, -6.0), 0.25, self.app_style.distal_light_color),
                                    Shape::circle(Pos2::new(4.0, -6.0), 0.25, self.app_style.distal_light_color)
                                ];
                                trunk = vec![
                                    Shape::circle(Pos2::new(0.0, 3.0), 0.25, self.app_style.trunk_light_color),
                                    Shape::circle(Pos2::new(0.0, 5.0), 0.25, self.app_style.trunk_light_color),
                                    Shape::circle(Pos2::new(0.0, 7.0), 0.25, self.app_style.trunk_light_color),
                                    Shape::circle(Pos2::new(0.0, 9.0), 0.25, self.app_style.trunk_light_color)
                                ];
                                tuft = vec![
                                    Shape::circle(Pos2::new(-1.0, 11.0), 0.25, self.app_style.tuft_light_color),
                                    Shape::circle(Pos2::new(1.0, 11.0), 0.25, self.app_style.tuft_light_color),
                                    Shape::circle(Pos2::new(-1.0, 13.0), 0.25, self.app_style.tuft_light_color),
                                    Shape::circle(Pos2::new(1.0, 13.0), 0.25, self.app_style.tuft_light_color),
                                    Shape::circle(Pos2::new(-3.0, 12.0), 0.25, self.app_style.tuft_light_color),
                                    Shape::circle(Pos2::new(3.0, 12.0), 0.25, self.app_style.tuft_light_color),
                                    Shape::circle(Pos2::new(-3.0, 14.0), 0.25, self.app_style.tuft_light_color),
                                    Shape::circle(Pos2::new(3.0, 14.0), 0.25, self.app_style.tuft_light_color),
                                    Shape::circle(Pos2::new(-4.0, 14.0), 0.25, self.app_style.tuft_light_color),
                                    Shape::circle(Pos2::new(4.0, 14.0), 0.25, self.app_style.tuft_light_color),
                                    Shape::circle(Pos2::new(-1.0, 15.0), 0.25, self.app_style.tuft_light_color),
                                    Shape::circle(Pos2::new(1.0, 15.0), 0.25, self.app_style.tuft_light_color),
                                    Shape::circle(Pos2::new(-5.0, 12.0), 0.25, self.app_style.tuft_light_color),
                                    Shape::circle(Pos2::new(5.0, 12.0), 0.25, self.app_style.tuft_light_color),
                                ];
                                axon = vec![
                                    Shape::circle(Pos2::new(0.0, -3.0), 0.25, self.app_style.axon_light_color),
                                    Shape::circle(Pos2::new(0.0, -5.0), 0.25, self.app_style.axon_light_color),
                                    Shape::circle(Pos2::new(0.0, -7.0), 0.25, self.app_style.axon_light_color),
                                    Shape::circle(Pos2::new(0.0, -9.0), 0.25, self.app_style.axon_light_color),
                                    Shape::circle(Pos2::new(0.0, -11.0), 0.25, self.app_style.axon_light_color),
                                    Shape::circle(Pos2::new(0.0, -13.0), 0.25, self.app_style.axon_light_color),
                                    Shape::circle(Pos2::new(-1.0, -15.0), 0.25, self.app_style.axon_light_color),
                                    Shape::circle(Pos2::new(1.0, -15.0), 0.25, self.app_style.axon_light_color),
                                    Shape::circle(Pos2::new(-1.0, -17.0), 0.25, self.app_style.axon_light_color),
                                    Shape::circle(Pos2::new(1.0, -17.0), 0.25, self.app_style.axon_light_color),
                                    Shape::circle(Pos2::new(-3.0, -16.0), 0.25, self.app_style.axon_light_color),
                                    Shape::circle(Pos2::new(3.0, -16.0), 0.25, self.app_style.axon_light_color),
                                    Shape::circle(Pos2::new(5.0, -16.0), 0.25, self.app_style.axon_light_color)
                                ];
                                synapse = vec![
                                    Shape::square(Pos2::new(-3.0, -18.0), 0.1, self.app_style.excitatory_light_color),
                                    Shape::square(Pos2::new(-1.0, -19.0), 0.1, self.app_style.fast_inhibitory_light_color),
                                    Shape::square(Pos2::new(5.0, -18.0), 0.1, self.app_style.slow_inhibitory_light_color),
                                    Shape::square(Pos2::new(1.0, -19.0), 0.1, self.app_style.synaptic_modulatory_light_color),
                                    Shape::square(Pos2::new(3.0, -18.0), 0.1, self.app_style.neural_modulatory_light_color),
                                ];
                            }

                            plot.show(ui, |plot_ui| {
                                let mut n = 0;
                                let line_color;

                                if self.dark_mode {
                                    line_color = self.app_style.line_dark_color;
                                }
                                else {
                                    line_color = self.app_style.line_light_color;
                                }

                                // Draw soma
                                match plot_ui.pointer_coordinate() {
                                    None => { plot_ui.line(soma.draw(false)); }
                                    Some(p) => {
                                        if soma.bounds.contains(p.to_pos2()) {
                                            plot_ui.line(soma.draw(true));
                                        }
                                        else {
                                            plot_ui.line(soma.draw(false));
                                        }
                                    }
                                }

                                // Draw proximal dendrites
                                n = 0;

                                for i in 0..proximal.len() {
                                    if i < 2 {
                                        plot_ui.line(Line::new(PlotPoints::new(
                                            vec![
                                                [soma.center.x as f64, soma.center.y as f64],
                                                [proximal[i].center.x as f64, proximal[i].center.y as f64]
                                            ]
                                        ))
                                            .style(LineStyle::Solid)
                                            .stroke(Stroke::new(0.33, line_color))
                                        );
                                    }
                                    else {
                                        plot_ui.line(Line::new(PlotPoints::new(
                                            vec![
                                                [proximal[n % 2].center.x as f64, proximal[n % 2].center.y as f64],
                                                [proximal[2 + n % 4].center.x as f64, proximal[2 + n % 4].center.y as f64]
                                            ]
                                        ))
                                            .style(LineStyle::Solid)
                                            .stroke(Stroke::new(0.33, line_color))
                                        );
                                    }

                                    match plot_ui.pointer_coordinate() {
                                        None => { plot_ui.line(proximal[i].draw(false)); }
                                        Some(p) => {
                                            if proximal[i].bounds.contains(p.to_pos2()) {
                                                plot_ui.line(proximal[i].draw(true));
                                            }
                                            else {
                                                plot_ui.line(proximal[i].draw(false));
                                            }
                                        }
                                    }
                                    n += 1;
                                }

                                // Draw distal dendrites
                                n = 0;

                                for i in 0..distal.len() {
                                    plot_ui.line(Line::new(PlotPoints::new(
                                        vec![
                                            [proximal[2 + n % 4].center.x as f64, proximal[2 + n % 4].center.y as f64],
                                            [distal[n % 8].center.x as f64, distal[n % 8].center.y as f64]
                                        ]
                                    ))
                                        .style(LineStyle::Solid)
                                        .stroke(Stroke::new(0.33, line_color))
                                    );

                                    match plot_ui.pointer_coordinate() {
                                        None => { plot_ui.line(distal[i].draw(false)); }
                                        Some(p) => {
                                            if distal[i].bounds.contains(p.to_pos2()) {
                                                plot_ui.line(distal[i].draw(true));
                                            }
                                            else {
                                                plot_ui.line(distal[i].draw(false));
                                            }
                                        }
                                    }
                                    n += 1;
                                }

                                // Draw apical trunk dendrites
                                for i in 0..trunk.len() {
                                    if i == 0 {
                                        plot_ui.line(Line::new(PlotPoints::new(
                                            vec![
                                                [soma.center.x as f64, soma.center.y as f64],
                                                [trunk[i].center.x as f64, trunk[i].center.y as f64]
                                            ]
                                        ))
                                            .style(LineStyle::Solid)
                                            .stroke(Stroke::new(0.33, line_color))
                                        );
                                    }
                                    else {
                                        plot_ui.line(Line::new(PlotPoints::new(
                                            vec![
                                                [trunk[i - 1].center.x as f64, trunk[i - 1].center.y as f64],
                                                [trunk[i].center.x as f64, trunk[i].center.y as f64]
                                            ]
                                        ))
                                            .style(LineStyle::Solid)
                                            .stroke(Stroke::new(0.33, line_color))
                                        );
                                    }

                                    match plot_ui.pointer_coordinate() {
                                        None => { plot_ui.line(trunk[i].draw(false)); }
                                        Some(p) => {
                                            if trunk[i].bounds.contains(p.to_pos2()) {
                                                plot_ui.line(trunk[i].draw(true));
                                            }
                                            else {
                                                plot_ui.line(trunk[i].draw(false));
                                            }
                                        }
                                    }
                                }

                                // Draw apical tuft dendrites
                                n = 0;

                                for i in 0..tuft.len() {
                                    if i < 2 {
                                        plot_ui.line(Line::new(PlotPoints::new(
                                            vec![
                                                [trunk[3].center.x as f64, trunk[3].center.y as f64],
                                                [tuft[i].center.x as f64, tuft[i].center.y as f64]
                                            ]
                                        ))
                                            .style(LineStyle::Solid)
                                            .stroke(Stroke::new(0.33, line_color))
                                        );
                                    }
                                    else if i < 6{
                                        plot_ui.line(Line::new(PlotPoints::new(
                                            vec![
                                                [tuft[n % 2].center.x as f64, tuft[n % 2].center.y as f64],
                                                [tuft[2 + n % 4].center.x as f64, tuft[2 + n % 4].center.y as f64]
                                            ]
                                        ))
                                            .style(LineStyle::Solid)
                                            .stroke(Stroke::new(0.33, line_color))
                                        );
                                    }
                                    else {
                                        plot_ui.line(Line::new(PlotPoints::new(
                                            vec![
                                                [tuft[2 + n % 4].center.x as f64, tuft[2 + n % 4].center.y as f64],
                                                [tuft[6 + n % 8].center.x as f64, tuft[6 + n % 8].center.y as f64]
                                            ]
                                        ))
                                            .style(LineStyle::Solid)
                                            .stroke(Stroke::new(0.33, line_color))
                                        );
                                    }

                                    match plot_ui.pointer_coordinate() {
                                        None => { plot_ui.line(tuft[i].draw(false)); }
                                        Some(p) => {
                                            if tuft[i].bounds.contains(p.to_pos2()) {
                                                plot_ui.line(tuft[i].draw(true));
                                            }
                                            else {
                                                plot_ui.line(tuft[i].draw(false));
                                            }
                                        }
                                    }
                                    n += 1;
                                }

                                // Draw axons
                                n = 0;

                                for i in 0..axon.len() {
                                    if i == 0 {
                                        plot_ui.line(Line::new(PlotPoints::new(
                                            vec![
                                                [soma.center.x as f64, soma.center.y as f64],
                                                [axon[i].center.x as f64, axon[i].center.y as f64]
                                            ]
                                        ))
                                            .style(LineStyle::Solid)
                                            .stroke(Stroke::new(0.33, line_color))
                                        );
                                    }
                                    else if i < 6 {
                                        plot_ui.line(Line::new(PlotPoints::new(
                                            vec![
                                                [axon[i - 1].center.x as f64, axon[i - 1].center.y as f64],
                                                [axon[i].center.x as f64, axon[i].center.y as f64]
                                            ]
                                        ))
                                            .style(LineStyle::Solid)
                                            .stroke(Stroke::new(0.33, line_color))
                                        );
                                    }
                                    else if i >= 6 && i < 8 {
                                        plot_ui.line(Line::new(PlotPoints::new(
                                            vec![
                                                [axon[5].center.x as f64, axon[5].center.y as f64],
                                                [axon[i].center.x as f64, axon[i].center.y as f64]
                                            ]
                                        ))
                                            .style(LineStyle::Solid)
                                            .stroke(Stroke::new(0.33, line_color))
                                        );
                                    }
                                    else if i < 12 {
                                        plot_ui.line(Line::new(PlotPoints::new(
                                            vec![
                                                [axon[6 + n % 2].center.x as f64, axon[6 + n % 2].center.y as f64],
                                                [axon[8 + n % 4].center.x as f64, axon[8 + n % 4].center.y as f64]
                                            ]
                                        ))
                                            .style(LineStyle::Solid)
                                            .stroke(Stroke::new(0.33, line_color))
                                        );
                                    }
                                    else {
                                        plot_ui.line(Line::new(PlotPoints::new(
                                            vec![
                                                [axon[i - 1].center.x as f64, axon[i - 1].center.y as f64],
                                                [axon[i].center.x as f64, axon[i].center.y as f64]
                                            ]
                                        ))
                                            .style(LineStyle::Solid)
                                            .stroke(Stroke::new(0.33, line_color))
                                        );
                                    }

                                    match plot_ui.pointer_coordinate() {
                                        None => { plot_ui.line(axon[i].draw(false)); }
                                        Some(p) => {
                                            if axon[i].bounds.contains(p.to_pos2()) {
                                                plot_ui.line(axon[i].draw(true));
                                            }
                                            else {
                                                plot_ui.line(axon[i].draw(false));
                                            }
                                        }
                                    }
                                    n += 1;
                                }

                                plot_ui.line(Line::new(PlotPoints::new(
                                    vec![
                                        [axon[10].center.x as f64, axon[10].center.y as f64],
                                        [synapse[0].center.x as f64, synapse[0].center.y as f64]
                                    ]
                                ))
                                    .style(LineStyle::Solid)
                                    .stroke(Stroke::new(0.33, line_color))
                                );

                                plot_ui.line(Line::new(PlotPoints::new(
                                    vec![
                                        [axon[8].center.x as f64, axon[8].center.y as f64],
                                        [synapse[1].center.x as f64, synapse[1].center.y as f64]
                                    ]
                                ))
                                    .style(LineStyle::Solid)
                                    .stroke(Stroke::new(0.33, line_color))
                                );

                                plot_ui.line(Line::new(PlotPoints::new(
                                    vec![
                                        [axon[12].center.x as f64, axon[12].center.y as f64],
                                        [synapse[2].center.x as f64, synapse[2].center.y as f64]
                                    ]
                                ))
                                    .style(LineStyle::Solid)
                                    .stroke(Stroke::new(0.33, line_color))
                                );

                                plot_ui.line(Line::new(PlotPoints::new(
                                    vec![
                                        [axon[9].center.x as f64, axon[9].center.y as f64],
                                        [synapse[3].center.x as f64, synapse[3].center.y as f64]
                                    ]
                                ))
                                    .style(LineStyle::Solid)
                                    .stroke(Stroke::new(0.33, line_color))
                                );

                                plot_ui.line(Line::new(PlotPoints::new(
                                    vec![
                                        [axon[11].center.x as f64, axon[11].center.y as f64],
                                        [synapse[4].center.x as f64, synapse[4].center.y as f64]
                                    ]
                                ))
                                    .style(LineStyle::Solid)
                                    .stroke(Stroke::new(0.33, line_color))
                                );

                                for i in 0..synapse.len() {
                                    match plot_ui.pointer_coordinate() {
                                        None => { plot_ui.line(synapse[i].draw(false)); }
                                        Some(p) => {
                                            if synapse[i].bounds.contains(p.to_pos2()) {
                                                plot_ui.line(synapse[i].draw(true));
                                            }
                                            else {
                                                plot_ui.line(synapse[i].draw(false));
                                            }
                                        }
                                    }
                                }
                            });
                        });
                    });
                });
                ui.end_row();
            });
    }
}

// App functions for NeuronModelerApp
impl App for NeuronModelerApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Create window frame
        self.window_frame(ctx, "Neuron Modeler", |ui| {

        });
    }

    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}