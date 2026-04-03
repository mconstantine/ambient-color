{% raw %}ZSH_THEME_GIT_PROMPT_CLEAN="%F{#40a02b}-%f"{% endraw %}
ZSH_THEME_GIT_PROMPT_DIRTY="%F{{ "{" ~ opposite.w400.bg ~ "}" }}*%f"

PROMPT="%F{{ "{" ~ opposite.w400.bg ~ "}" }}%n@%m%f {% raw %}%D{%a %d} %B%~%b %# "{% endraw %}
