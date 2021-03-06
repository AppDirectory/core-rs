include!("./_lib.rs");

#[cfg(test)]
mod tests {
    use super::*;

    use ::config;

    #[test]
    fn filesync_outgoing() {
        let handle = init();
        let password: String = config::get(&["integration_tests", "login", "password"]).unwrap();

        dispatch_ass(json!(["app:wipe-app-data"]));
        dispatch_ass(json!(["user:join", "slippyslappy@turtlapp.com", password]));
        dispatch_ass(json!(["sync:start"]));

        wait_on("profile:loaded");
        wait_on("profile:indexed");

        let profile_res = dispatch(json!(["profile:load"]));

        let user_id: String = jedi::get(&["user", "id"], &profile_res.d).unwrap();
        let space_id: String = jedi::get(&["spaces", "0", "id"], &profile_res.d).unwrap();
        let notejson = &json!({
            "title": "mai file LOL",
            "space_id": space_id,
            "user_id": user_id,
            "file": {
                "name": "slappy.json",
                "type": "application/json"
            }
        });
        let file_contents = String::from(r#"eyJuYW1lIjoic2xhcHB5IiwibG9jYXRpb24iOnsiY2l0eSI6InNhbnRhIGNydXp6eiJ9LCJlbmpveXMiOiJzaGFyaW5nIHNlbGZpZXMgd2l0aCBtYWkgaW5zdGFncmFtIGZvbGxvd2VycyEiLCJpbnN0YWdyYW1fZm9sbG93ZXJzIjpbXX0="#);
        let filejson = &json!({
            "data": file_contents,
        });
        let res = dispatch(json!([
            "profile:sync:model",
            "add",
            "note",
            notejson,
            filejson,
        ]));
        let note_id: String = jedi::get(&["id"], &res.d).unwrap();
        if res.e != 0 {
            panic!("bad response from profile:sync:model -- {:?}", res);
        }

        wait_on("sync:file:uploaded");

        dispatch_ass(json!(["profile:sync:model", "delete", "file", {"id": note_id}]));
        dispatch_ass(json!(["user:delete-account"]));
        end(handle);
    }
}

