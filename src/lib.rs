use std::time::Duration;
use yew::services::{ConsoleService, IntervalService, Task};
use yew::{Html, Callback, Component, ComponentLink, html, ShouldRender};

pub struct Model {
    link: ComponentLink<Self>,
    interval: IntervalService,
    console: ConsoleService,
    callback_tick: Callback<()>,
    job: Option<Box<dyn Task>>,
    message: Vec<&'static str>,
    _standalone: Box<dyn Task>,
}

pub enum Message {
    StartInterval,
    Cancel,
    Done,
    Tick,
}

impl Component for Model {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = |_| {
            println!("Example of standalone callback");
        };
        let mut interval = IntervalService::new();
        let handle = interval.spawn(Duration::from_secs(0), callback.into());

        Model {
            link: link.clone(),
            interval,
            console: ConsoleService::new(),
            callback_tick: link.callback(|_| Message::Tick),
            job: None,
            message: Vec::new(),
            _standalone: Box::new(handle),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::StartInterval => {
                {
                    let handle = self.interval.spawn(Duration::from_secs(1), self.callback_tick.clone());
                    self.job = Some(Box::new(handle));
                }
                self.message.clear();
                self.message.push("Interval started");
                self.console.log("Interval started");
            },
            Message::Cancel => {
                self.job.take();
                self.message.push("canceled");
                self.console.warn("canceled");
                self.console.assert(self.job.is_none(), "Job still exists");
            }, 
            Message::Done => {
                self.message.push("Done");
                self.console.group();
                self.console.info("Done");
                self.console.time_named_end("Timer");
                self.console.group_end();
                self.job = None;
            },
            Message::Tick => {
                self.console.count_named("Tick");
            }
        }
        true
    }
    
    fn view(&self) -> Html {
        let view_message = |message| {
            html! { <p>{ message }</p> }
        };
        let has_job = self.job.is_some();
        html! {
            <div>
                <button disabled = has_job onclick = self.link.callback(|_| Message::StartInterval)>{ "START" }</button>
                <button disabled = !has_job onclick = self.link.callback(|_| Message::Cancel)>{ "STOP" }</button>
                
                <div>
                    { for self.message.iter().map(view_message) }
                </div>
            </div>
        }
    }
}
