use yew::prelude::*;

pub struct ButtonName {
    class: String,
    title: String,
    onsignal: Callback<()>,
}

pub enum Msg {
    Clicked,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub class: String,
    pub title: String,
    #[props(required)]
    pub onsignal: Callback<()>,
}

impl Component for ButtonName {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        ButtonName {
            class: props.class,
            title: props.title,
            onsignal: props.onsignal,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                self.onsignal.emit(());
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.class = props.class;
        self.title = props.title;
        self.onsignal = props.onsignal;
        true
    }
}

impl Renderable<ButtonName> for ButtonName {
    fn view(&self) -> Html<Self> {
        html! {
            <button class={&self.class} onclick=|_| Msg::Clicked>
            <i class="fa fa-desktop"></i>
            { &self.title }</button>
        }
    }
}
