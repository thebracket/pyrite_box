use super::ModuleResource;
use crate::game_states::player_movement::PlayerMoveRequest;
use crate::module::game_events::EventPicker;
use crate::module::game_events::GameEvent;
use crate::module::game_events::GameEventStep;
use bevy_egui::egui;
use bevy_egui::EguiContext;

pub fn events(egui_context: &EguiContext, module_res: &mut ModuleResource) {
    if module_res.show_events {
        egui::Window::new("Events")
            .title_bar(true)
            .show(egui_context.ctx(), |ui| {
                ui.label("New Event Tag");
                ui.text_edit_singleline(&mut module_res.new_event_tag);
                if ui.button("Add Event").clicked() {
                    let new_event = GameEvent {
                        tag: module_res.new_event_tag.clone(),
                        steps: Vec::new(),
                    };
                    module_res.module.events.events.push(new_event);
                }
                ui.separator();
                let mut edit_event = None;
                for e in module_res.module.events.events.iter() {
                    if ui.button(&e.tag).clicked() {
                        edit_event = Some(e.tag.clone());
                    }
                }
                if edit_event.is_some() {
                    module_res.editing_event = edit_event;
                }
            });
    }
}

pub fn event_editor(egui_context: &EguiContext, module_res: &mut ModuleResource) {
    if module_res.editing_event.is_some() {
        let tag = module_res.editing_event.clone().unwrap();
        let mut next_step = module_res.new_event_step;
        if let Some(event) = module_res
            .module
            .events
            .events
            .iter_mut()
            .find(|e| e.tag.eq(&tag))
        {
            egui::Window::new(format!("Event: {}", tag))
                .title_bar(true)
                .resizable(true)
                .show(egui_context.ctx(), |ui| {
                    ui.text_edit_singleline(&mut event.tag);

                    egui::ComboBox::from_label("New Step").show_ui(ui, |ui| {
                        ui.selectable_value(&mut next_step, EventPicker::LogText, "Log");
                        ui.selectable_value(&mut next_step, EventPicker::ClearLog, "Clear Log");
                        ui.selectable_value(&mut next_step, EventPicker::CallEvent, "Call");
                        ui.selectable_value(&mut next_step, EventPicker::PauseMs, "Pause Delay MS");
                        ui.selectable_value(&mut next_step, EventPicker::MovePlayer, "Move Player");
                    });

                    if ui.button("Add Step").clicked() {
                        match next_step {
                            EventPicker::LogText => {
                                event.steps.push(GameEventStep::LogText {
                                    text: "Hello".to_string(),
                                    color: None,
                                });
                            }
                            EventPicker::ClearLog => {
                                event.steps.push(GameEventStep::ClearLog);
                            }
                            EventPicker::PauseMs => {
                                event.steps.push(GameEventStep::PauseMs(33));
                            }
                            EventPicker::CallEvent => {
                                event.steps.push(GameEventStep::CallEvent(String::new()));
                            }
                            EventPicker::MovePlayer => {
                                event.steps.push(GameEventStep::MovePlayer(
                                    PlayerMoveRequest::Forwards,
                                    1000,
                                ));
                            }
                        }
                    }

                    // List steps
                    for (line, e) in event.steps.iter_mut().enumerate() {
                        match e {
                            GameEventStep::LogText { text, .. } => {
                                ui.label(format!("{} : Log Text", line));
                                ui.text_edit_singleline(text);
                            }
                            GameEventStep::ClearLog => {
                                ui.label(format!("{} : Clear Log", line));
                            }
                            GameEventStep::PauseMs(ms) => {
                                ui.label(format!("{} : Pause Ms", line));
                                ui.add(egui::Slider::new(ms, 1..=10000));
                            }
                            GameEventStep::CallEvent(tag) => {
                                ui.label(format!("{} : Call Event", line));
                                ui.text_edit_singleline(tag);
                            }
                            GameEventStep::MovePlayer(..) => {
                                ui.label(format!("{} : Move Player", line));
                                // TODO: Editor support
                            }
                            GameEventStep::ChangeMap { .. } => {
                                ui.label(format!("{} : Change Map", line));
                                // TODO: Editor support
                            }
                            GameEventStep::InputBranch { .. } => {
                                ui.label("Input branch");
                            }
                            GameEventStep::Sprite(..) => {
                                ui.label("Sprite Action");
                            }
                        }
                    }
                });
        }
        module_res.new_event_step = next_step;
    }
}
