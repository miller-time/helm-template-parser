# helm-template-parser

A parser for reading the YAML files within your Helm [templates][1] directory
(that contain Helm [template expressions][2]).

The Helm expressions themselves are not evaluated, but this allows structured
interactions with the files here, e.g. programmatic modifications.

[1]: https://helm.sh/docs/chart_best_practices/templates/
[2]: https://helm.sh/docs/chart_template_guide/function_list/
