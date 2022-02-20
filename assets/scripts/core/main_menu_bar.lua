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
    local t = require("test.test_require");
    if t then
        t.test();
    else
        print("req failed");
    end
end

function test_vec()
    if vector then
        if vector.new_vec3 then
            local a = vector.new_vec3();
            a.x = 1;
            a.y = 2;
            a.z = 3;
            local b = vector.new_vec3();
            b.x = 4;
            b.y = 5;
            b.z = 6;
            if a and b then
                local c = a + b;
                print(c);
            else
                print("a and/or b is nil");
            end
        else
            print("vector does not have new_vec3");
        end
    else
        print("vector does not exist");
    end
end
