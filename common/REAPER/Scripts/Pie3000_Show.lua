local r = reaper
local script_path = r.GetResourcePath() .. "/Scripts/Sexan_Scripts/Pie3000/"
package.path = script_path .. "?.lua;"

require('PieUtils')

local menu_file = script_path .. "menu_file.txt"

local MENUS = ReadFromFile(menu_file) or {}
for i = 1, #MENUS do
    if MENUS[i].guid == "{63696A1A-6E7D-ACE5-5ECB-05E44FDC95BB}" then
        STANDALONE_PIE = MENUS[i]
        break
    end
end
if STANDALONE_PIE then
    require('Sexan_Pie3000')
else
    r.ShowConsoleMsg("Menu does not exist")
end
