if app.apiVersion < 1 then
    return app.alert("Your aseprite is too old. Please update your aseprite")
end

local dlg = Dialog()
dlg:file{
    id = "choose_save_loc",
    save = true,
    label = "choose location",
    title = "exporting to wasm4",
    filename = "objects.rs",
    filetypes = ".rs"
}:show()

local file_name = dlg.data.choose_save_loc

local cel = app.activeCel
if not cel then
    return app.alert("No active image!")
end

local result_string = ""

local pc = app.pixelColor
local img = cel.image:clone()

local object_count = 0

math.randomseed(os.clock())

for j = 0, 79, 1 do
    for i = 0, 79, 1 do
        local pix = img:getPixel(i, j)
        local r = pc.rgbaR(pix)
        local g = pc.rgbaG(pix)
        local b = pc.rgbaB(pix)

        local x_coord = i * 2
        local y_coord = j * 2

        if r == 231 and g == 35 and b == 143 then
            object_count = object_count + 1
            local rnd_val = math.random(5)
            if rnd_val == 1 then
                result_string = result_string ..
                        "    (Object::Tree(TreeType::Pine), ("
            elseif rnd_val == 2 then
                result_string = result_string ..
                        "    (Object::Tree(TreeType::Oak), ("
            elseif rnd_val == 3 then
                result_string = result_string ..
                        "    (Object::Tree(TreeType::LittleOak), ("
            else
                result_string = result_string ..
                        "    (Object::Tree(TreeType::LittlePine), ("
            end

            result_string = result_string ..
                x_coord .. "," .. y_coord .. ")),\n"
        elseif r == 142 and g == 255 and b == 141 then
            object_count = object_count + 1
            result_string = result_string ..
                "    (Object::Unit(UnitType::Human(HumanType::Worker)), (" ..
                x_coord .. "," .. y_coord .. ")),\n"
        elseif r == 158 and g == 19 and b == 96 then
            object_count = object_count + 1
            result_string = result_string ..
                "    (Object::House(HouseType::BigHouse), (" ..
                x_coord .. "," .. y_coord .. ")),\n"
        end
    end
end

result_string = "const OBJECTS: [(Object, (i16, i16));" .. object_count .. "] = [\n" ..
    result_string ..
    "];"

local file = io.open (file_name , "w+")
io.output(file)
io.write(result_string)
io.close(file)
