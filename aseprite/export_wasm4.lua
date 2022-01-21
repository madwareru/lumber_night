if app.apiVersion < 1 then
    return app.alert("Your aseprite is too old. Please update your aseprite")
end

local dlg = Dialog()
dlg
  :file{
    id = "choose_save_loc",
    save = true,
    label = "choose location",
    title = "exporting to wasm4",
    filename = "sprites.rs",
    filetypes = ".rs"
  }
  :show()

local file_name = dlg.data.choose_save_loc

local cel = app.activeCel
if not cel then
    return app.alert("No active image!")
end

local result_string =
    "use super::tile::Tile;\n" ..
	"\n"..
    "pub const TILES: [Tile; 64] = [\n"

local pc = app.pixelColor
local img = cel.image:clone()

for j = 0, 3, 1 do
    for i = 0, 15, 1 do
	    local s1 = "        colors: [\n"
	    local s2 = "        opacity_mask: [\n"
        for y = 0, 7, 1 do
	        local pix_s = ""
	        local opac_s = ""
            for x = 0, 7, 1 do
                local pix = img:getPixel(i * 8 + x, j * 8 + y)
		        local r = pc.rgbaR(pix)
		        local g = pc.rgbaG(pix)
		        local b = pc.rgbaB(pix)
				if r == 35 and g == 46 and b == 69 then
					pix_s = "00" .. pix_s
					opac_s = "11" .. opac_s
                elseif r == 60 and g == 93 and b == 117 then
                    pix_s = "01" .. pix_s
                    opac_s = "11" .. opac_s
		        elseif r == 94 and g == 178 and b == 160 then
                    pix_s = "10" .. pix_s
		            opac_s = "11" .. opac_s
		        elseif pc.rgbaA(pix) == 0 then
		            pix_s = "00" .. pix_s
		            opac_s = "00" .. opac_s
		        else
                    pix_s = "11" .. pix_s
		            opac_s = "11" .. opac_s
	            end
		        if x == 3 then
		            s1 = s1 .. "        0b" .. pix_s .. ", "
		            s2 = s2 .. "        0b" .. opac_s .. ", "
					pix_s = ""
					opac_s = ""
		        elseif x == 7 then
					s1 = s1 .. "0b" .. pix_s .. ",\n "
					s2 = s2 .. "0b" .. opac_s .. ",\n "
		        end
	        end
	    end
		result_string = result_string .. "    Tile{\n" .. s1 .. "        ],\n" .. s2 .. "        ]},\n"
	end
end
result_string = result_string .. "    ];"

local file = io.open (file_name , "w+")
io.output(file)
io.write(result_string)
io.close(file)
