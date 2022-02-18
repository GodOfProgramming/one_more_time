function initialize(_data)
    Logger:debug("initialized");
end

function update()
    Logger:debug("updated");
end

function test_fn()
    UiManager:open("test_window", "test_window_instance");
end

function rename_window()
    local test_window_doc = UiManager:get("test_window_instance");
    if test_window_doc ~= nil then
        local win = test_window_doc:get_element_by_id("test.test_window");
        if win ~= nil then
            print("setting attrib");
            win:set_attrib("title", "New Title");
        else
            print("win nil");
        end
    else
        print("test window doc nil");
    end
end

function on_exit()
    Logger:info("quitting game");
    App:request_exit();
end
