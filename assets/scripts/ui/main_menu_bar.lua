require "lib.class";
require 'lib.ui';
local inspect = require 'lib.inspect'

local MainMenuBar = class([[MainMenuBar]], { module = ui });

function MainMenuBar:update()
    Logger:debug("updated");
end

function MainMenuBar:show_or_hide_profiler()
    Settings:game():show_or_hide_profiler();
end

function MainMenuBar:show_or_hide_demo_window()
    Settings:game():show_or_hide_demo_window();
end

function MainMenuBar:on_exit()
    Logger:info("quitting game");
    App:request_exit();
end

function MainMenuBar:list_ui()
    for idx, item in ipairs(UiManager:list()) do
        print(idx .. ": " .. item);
    end
end

function MainMenuBar:create_window()
    local handle = UiManager:open("test.test_window", "test_window_instance");
    if not handle then
        Logger:warning("handle is nil");
    end
end

function MainMenuBar:rename_window()
    local test_window_doc = UiManager:get("test_window_instance");
    if test_window_doc ~= nil then
        local win = test_window_doc:get_element_by_id("test_window");
        if win ~= nil then
            win:set_attrib("title", "New Title");
        else
            print("win nil");
        end
    else
        print("test window doc nil");
    end
end

function MainMenuBar:test()
    local entity = Map:lookup_entity(self.char_id);
    entity:dispose();
end

function MainMenuBar:test_class()
    local m = {}
    local Parent = class([[Parent]], {
      module = m,
      body = {
        foo = 1,
      }
    });

    function Parent:hello()
        print("parent hello");
    end

    local Child = class([[Child]], {
      extends = Parent,
      module = m,
      body = {
        bar = 2,
      }
    });

    function Child:hello()
        print("child hello");
    end

    local parent = Parent:new();
    local child = Child:new();

    print("parent");
    parent:hello();

    print("child");
    child:hello();
end
