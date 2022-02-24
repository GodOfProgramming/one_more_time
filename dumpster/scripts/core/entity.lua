require "lib.class";
require "lib.core";
local inspect = require "lib.inspect"

local Square = class([[Square]], {
  module = core,
  extends = Class,
  body = {
    count = 0
  }
});

function Square:update(handle)
    if self.count == 0 then
        print("update");
        local bar = UiManager:get("debug_main_menu_bar");
        if bar then
            bar.data.char_id = handle.id;
            print(inspect(bar.data));
        end
    end

    self.count = self.count + 1;
end
