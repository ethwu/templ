# templ #
Handlebars templating for generating files.

## Installation ##
Build and install with:
```sh
cargo install --path .
```

## Usage ##

```sh
templ [FLAGS] [OPTIONS] <TEMPLATE>
```

<dl>
    <dt><code>&lt;TEMPLATE></code></dt>
    <dd><p>Path to a template file. The template _must_ have the extension&nbsp;<code>.hbs</code>.</p></dd>
</dl>

### Flags ###
<dl>
    <dt><code>-h</code>, <code>--help</code></dt>
    <dd><p>Display help. Use <code>--help</code> for more information.</p></dd>
    <dt><code>-s</code>, <code>--stdout</code></dt>
    <dd><p>Emit the rendered template to stdout instead of writing to a file.</p></dd>
    <dt><code>-V</code>, <code>--version</code></dt>
    <dd><p>Display version.</p></dd>
</dl>

### Options ###
<dl>
    <dt><code>-d</code>, <code>--data &lt;path></code></dt>
    <dd><p>Path to a TOML&nbsp;file containing the data to interpolate. If not specified, either a file with the same name as the template with the <code>.hbs</code>&nbsp;extension replaced with&nbsp;<code>.toml</code> (e.g.,&nbsp;<code>template.txt.hbs</code> and&nbsp;<code>template.txt.toml</code>) or&nbsp;<code>template.toml</code> in the current working directory will be used instead.</p></dd>
    <dt><code>-o</code>, <code>--output &lt;path></code></dt>
    <dd><p>Select a specific output file path. If left unspecified, the output file path will be generated from the template file name:</p>
        <ul>
            <li>If a table <code>filename</code> exists in the data file, the name of the template file is used as a Handlebars&nbsp;template, and keys from the <code>filename</code> table are used to render the file name.</li>
            <li>If the key <code>filename</code> exists in the data file, any instances of the string <code>template</code> in the template file name are replaced with the value of <code>filename</code>.</li>
            <li>If the key <code>filename</code> is left undefined, no such substitution happens.</li>
        </ul>
    <p>The <code>.hbs</code> file extension is stripped from the file name.</p>
    <p>For example, with the data file</p>
    <pre lang="toml"><code># template.md.toml
name = "Alice"
age = 22
profession = "Cryptographer"
<br />
filename = "profile"</code></pre>
    <p>and the template file</p>
    <pre lang="hbs"><code>&lt;!-- template.md.hbs -->
# {{name}} #
{{name}} is a {{age}} year old {{profession}}.</code></pre>
    <p>the automatically generated file name would be <code>profile.md</code>. If the <code>filename</code> key were left out, the name would be <code>template.md</code>. If the template file were named <code>input.md.hbs</code> instead of <code>template.md.hbs</code>, the output file would be&nbsp;<code>input.md</code>.</p>
    <p>With the data file</p>
    <pre lang="toml"><code># template.toml
secret = 42
[filename]
secret = "noodle"
flavor = "spicy"</code></pre>
    <p>and the template file</p>
    <pre lang="hbs"><code># {{secret}}-reveal ({{flavor}}).py.hbs
assert "{{secret}}" == 42</code></pre>
    <p>the automatically generated file name would be <code>secret-reveal (spicy).py</code>.</p></dd>
</dl>
