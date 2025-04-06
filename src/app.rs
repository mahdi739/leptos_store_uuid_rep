use leptos::{prelude::*, task::spawn_local};
use reactive_stores::{Field, Store, StoreFieldIterator};
use uuid::Uuid;

#[derive(Store, Default, Clone)]
pub struct State {
  #[store(key:usize = |session| session.id)]
  sessions: Vec<Session>,
  selected_session: Option<Field<Session>>,
}

#[derive(Store, Default, PartialEq, Eq, Hash, Clone)]
pub struct Session {
  id: usize,
  #[store(key:Uuid = |message| message.id)]
  messages: Vec<Message>,
}

#[derive(Store, Default, PartialEq, Eq, Hash, Clone)]
pub struct Message {
  id: Uuid,
  #[store(key:String = |part| part.clone())]
  parts: Vec<String>,
}

#[component]
pub fn App() -> impl IntoView {
  let state = Store::new(State {
    selected_session: None,
    sessions: vec![
      Session { id: 0, messages: vec![Message { id: Uuid::new_v4(), parts: vec![] }] },
      Session { id: 1, messages: vec![Message { id: Uuid::new_v4(), parts: vec![] }] },
      Session { id: 2, messages: vec![Message { id: Uuid::new_v4(), parts: vec![] }] },
      Session { id: 3, messages: vec![Message { id: Uuid::new_v4(), parts: vec![] }] },
      Session { id: 4, messages: vec![Message { id: Uuid::new_v4(), parts: vec![] }] },
    ],
  });

  view! {
    <button on:click=move |_| {
      let length = state.sessions().get().len();
      state
        .sessions()
        .update(|f| {
          f.push(Session {
            id: length + 1,
            messages: vec![
              Message {
                id: Uuid::new_v4(),
                parts: vec![],
              },
            ],
          })
        });
    }>"ADD SESSION"</button>
    {move || {
      state
        .selected_session()
        .get()
        .map(|selected_sessioon| {
          view! {
            <button on:click=move |_| {
              let length = selected_sessioon.get().messages.last().unwrap().parts.len() + 1;
              let message = selected_sessioon.messages().iter_unkeyed().last().unwrap();
              message.parts().update(|m| m.push(format!("{length}")));
            }>"Count"</button>
          }
        })
    }}
    <For each=move || state.sessions() key=|session| session.id().get() let:session>
      <div>
        <label>
          <input
            type="radio"
            checked=state
              .selected_session()
              .try_get()
              .flatten()
              .map(|s| s.id().get() == session.id().get())
            name="session"
            on:click=move |_| {
              state.selected_session().set(Some(session.into()));
            }
          />
          <b>"Session " {move || session.id().get().to_string()} ": "</b>
        </label>
        <For each=move || session.messages() key=|message| message.id().get() let:message>
          <For each=move || message.parts() key=|part| part.get() let:part>
            {move || part.get()}
          </For>
        </For>
      </div>
    </For>
  }
}
