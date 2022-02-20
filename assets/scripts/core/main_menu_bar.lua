function initialize(_data)
    Logger:debug("initialized");
end

function update()
    Logger:debug("updated");
end

function show_or_hide_profiler()
    Settings:game():show_or_hide_profiler();
end

function show_or_hide_demo_window()
    Settings:game():show_or_hide_demo_window();
end

function on_exit()
    Logger:info("quitting game");
    App:request_exit();
end

function list_ui()
    for idx, item in ipairs(UiManager:list()) do
        print(idx .. ": " .. item);
    end
end

function create_window()
    local handle = UiManager:open("test.test_window", "test_window_instance");
    if not handle then
        Logger:warning("handle is nil");
    end
end

function rename_window()
    local test_window_doc = UiManager:get("test_window_instance");
    if test_window_doc ~= nil then
        local win = test_window_doc:get_element_by_id("test_window");
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

function test()
    local cpml = require("cpml.init");
    if cpml then
        local a = cpml.vec3.new(1, 2, 3);
        local b = cpml.vec3.new(4, 5, 6);
        local c = a + b;

        print(c);
    else
        print("req failed");
    end
end
