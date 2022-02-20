require "lib.class";
local inspect = require "lib.inspect";

Square = class { body = { count = 0 } };

function Square:update()
    if self.count == 0 then
        print("update");
    end

    self.count = self.count + 1;
end
