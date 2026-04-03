-- Reference: https://daisyui.com/theme-generator
local bg_800 = "{{ primary_800 }}"
local bg_900 = "{{ primary_900 }}"
local primary = "{{ primary_500 }}"
local success = "#01df72" -- green 400
local success_content = "#022d14" -- green 950

local colors = {
	bg_dim = "{{ neutral_950 }}",
	bg = "{{ primary_950 }}",
	bg_950 = "{{ primary_950 }}",
	bg_900 = bg_900,
	bg_800 = bg_800,
	bg_500 = "{{ primary_500 }}",
	fg = "{{ primary_50 }}",
	fg_dim = "{{ primary_200 }}",
	primary = primary,
	primary_content = "{{ primary_500_foreground }}",
	secondary_50 = "{{ secondary_50 }}",
	secondary = "{{ secondary_400 }}",
	secondary_800 = "{{ secondary_800 }}",
	secondary_content = "{{ secondary_950 }}",
	success = success,
	success_500 = "#00c850",
	success_800 = "#00642e",
	success_content = success_content,
	neutral = "{{ neutral_400 }}",
	error = "#ff6266",
	error_800 = "#9d0410",
	error_content = "#440607", -- red 950
	visual_bg = bg_900,
	info = success,
	info_content = success_content,
	warning = primary,
	hint = "{{ tertiary_500 }}",
	hint_content = "{{ tertiary_500_foreground }}",
}

return colors
