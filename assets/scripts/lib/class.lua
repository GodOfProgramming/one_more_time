local inspect = require("lib.inspect");

function extend(parent, child)
    for k, v in pairs(parent) do
        if (type(v) == 'table') then
            child[k] = merge(parent[k], child[k]);
        else
            child[k] = v;
        end
    end
    return child;
end

-- metaclass

Class = {}

-- initializer

function Class:new(o)
    o = o or {};
    setmetatable(o, self);
    self.__index = self;
    self.class = Class;
    return o;
end

-- public methods

function Class:to_string()
    return inspect(self);
end

function Class:print()
    print(self:to_string());
end

-- Class creator

--[[
  @brief Create a new class
  <br />
  @param args { super: Class, body: {...} }
--]]
function class(args)
    args = args or {};
    args.super = args.super or Class;
    args.body = args.body or {};
    return extend(args.super, args.body);
end
