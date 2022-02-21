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
    o = o == nil and {} or o;
    setmetatable(o, self);
    self.__index = self;
    self.class = Class;
    return o;
end

-- public methods

function Class:to_string()
    return inspect(self);
end

-- Class creator

--[[
  @brief Create a new class
  <br />
  @param args { super: Class, body: {...} }
--]]
function class(name, args)
    args = args == nil and {} or args;
    args.extends = args.extends or Class;
    args.body = args.body or {};
    module = args.module == nil and _G or args.module;
    new_class = extend(args.extends, args.body);
    module[name] = new_class;
    return new_class;
end
