-- Reference: https://daisyui.com/theme-generator
local bg_800 = "#193ab7"
local bg_900 = "#1a368b"
local success = "#01df72" -- green 400
local primary = "#ffc91d" -- og aranciollo
local success_content = "#022d14" -- green 950

local colors = {
	bg_dim = "#010515", -- slate 950
	bg = "#040939", -- blue 950 - 10% lightness
	bg_950 = "#162455",
	bg_900 = bg_900,
	bg_800 = bg_800,
	bg_500 = "#2a7eff",
	fg = "#bddafe", -- blue 200
	fg_dim = "#4ea0ff", -- blue 400
	primary = primary,
	primary_content = "#793205", -- amber 50
	secondary_50 = "#f8f3fd",
	secondary = "#c079ff", -- purple 400
	secondary_800 = "#6b0bad",
	secondary_content = "#3c0366", -- purple 950
	success = success,
	success_500 = "#00c850",
	success_800 = "#00642e",
	success_content = success_content, -- green 950
	neutral = "#8fa0b8", -- slate 400
	error = "#ff6266", -- red 400
	error_800 = "#9d0410",
	error_content = "#440607", -- red 950
	visual_bg = bg_900,
	info = success,
	info_content = success_content,
	warning = primary,
	hint = "#00baa6", -- teal 500
	hint_content = "#002d2c", -- teal-950
}

return colors
