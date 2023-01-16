use monaco::{
    api::{CodeEditorOptions, TextModel},
    sys::editor::BuiltinTheme,
    yew::CodeEditor,
};
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct EditorProps {
    pub text_model: TextModel,
}

#[function_component(MarkdownEditor)]
pub fn editor(EditorProps { text_model }: &EditorProps) -> Html {
    let options = CodeEditorOptions::default()
        .with_language("markdown".to_owned())
        .with_value("# Hello, World!".to_owned())
        .with_builtin_theme(BuiltinTheme::VsDark)
        .with_automatic_layout(true);

    html! {
        <CodeEditor
            classes={"full-height"}
            options={options.to_sys_options()}
            model={text_model.clone()}
        />
    }
}
