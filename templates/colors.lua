-- Reference: https://daisyui.com/theme-generator
local bg_800 = "{{ primary.w800.bg }}"
local bg_900 = "{{ primary.w900.bg }}"
local primary = "{{ opposite.w400.bg }}"
local success = "#01df72" -- green 400
local success_content = "#022d14" -- green 950

local colors = {
	bg_dim = "{{ neutral.w950.bg }}",
	bg = "{{ primary.w950.bg }}",
	bg_950 = "{{ primary.w950.bg }}",
	bg_900 = bg_900,
	bg_800 = bg_800,
	bg_500 = "{{ primary.w500.bg }}",
	fg = "{{ primary.w950.fg }}",
	fg_dim = "{{ neutral.w950.fg }}",
	primary = primary,
	primary_content = "{{ opposite.w400.fg }}",
	secondary_50 = "{{ secondary.w50.bg }}",
	secondary = "{{ secondary.w400.bg }}",
	secondary_800 = "{{ secondary.w800.bg }}",
	secondary_content = "{{ secondary.w950.fg }}",
	success = success,
	success_500 = "#00c850",
	success_800 = "#00642e",
	success_content = success_content,
	neutral = "{{ neutral.w400.bg }}",
	error = "#ff6266",
	error_800 = "#9d0410",
	error_content = "#440607", -- red 950
	visual_bg = bg_900,
	info = success,
	info_content = success_content,
	warning = primary,
	hint = "{{ tertiary.w500.bg }}",
	hint_content = "{{ tertiary.w500.fg }}",
}

return colors
