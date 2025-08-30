# Forge-Tree 

> **"Why spend hours creating folder structures when you can forge them in seconds?"**

Ever stared at a project structure in documentation and thought *"Ugh, I have to create all these folders and files manually?"* Yeah, we've all been there. That's exactly why **Forge-Tree** exists!

## The Problem We All Face 

You know the drill:
```graphql
my-awesome-project/
├── src/
│   ├── components/
│   │   ├── Header.js
│   │   └── Footer.js
│   ├── utils/
│   │   └── helpers.js
│   └── index.js
├── tests/
│   └── app.test.js
├── package.json
└── README.md
```

And then you spend the next 10 minutes going:
- *Right-click → New Folder → "src"*
- *Right-click → New Folder → "components"*
- *Right-click → New File → "Header.js"*
- *Repeat 47 more times...*
- *Contemplate life choices*

## The Solution That'll Make You Smile 

**Forge-Tree** takes that ASCII art structure and magically transforms it into real folders and files. No clicking. No tedious manual work. Just pure, beautiful automation.

## Installation (30 Seconds to Happiness)

```bash
cargo install forge-tree
```

*That's it. You're now a project structure wizard.* 🧙‍♂️

## See It In Action

Create a file called `my-dream-project.txt`:
```graphql
my-dream-project/
├── frontend/
│ ├── src/
│ │ ├── components/
│ │ │ ├── Header.jsx
│ │ │ ├── Footer.jsx
│ │ │ └── Sidebar.jsx
│ │ ├── pages/
│ │ │ ├── Home.jsx
│ │ │ └── About.jsx
│ │ ├── utils/
│ │ │ └── api.js
│ │ └── App.jsx
│ ├── package.json
│ └── README.md
├── backend/
│ ├── src/
│ │ ├── routes/
│ │ │ ├── auth.rs
│ │ │ └── api.rs
│ │ ├── models/
│ │ │ └── user.rs
│ │ ├── main.rs
│ │ └── lib.rs
│ ├── Cargo.toml
│ └── README.md
├── docs/
│ ├── api.md
│ └── setup.md
└── docker-compose.yml
```
Now watch the magic happen:
```bash
forge-tree forge my-dream-project.txt
```

***BOOM!** 💥 Your entire project structure appears like you're some kind of terminal sorcerer.*

```bash
🏗️ Forging project structure...
📖 Parsing structure from: my-dream-project.txt
🌳 Root: my-dream-project
📁 Items: 23

✨ [████████████████████████████████] 23/23 Processing docker-compose.yml

✅ Project 'my-dream-project' forged successfully at ./my-dream-project
```


## Why You'll Love It

###  **Lightning Fast**
Built with Rust because life's too short for slow tools. We're talking milliseconds, not minutes.

###  **Actually Beautiful CLI**
None of that boring terminal output. We've got colors, progress bars, and emojis because we're not savages.

###  **Smart AF**
- **Auto-detects** files vs folders (`.js` = file, no extension = probably a folder)
- **Handles nesting** like a boss (go 50 levels deep, we don't judge)
- **Validates** your structure before forging (catches typos before you do)

###  **Safe by Default**
Won't overwrite your existing files unless you explicitly tell it to with `--force`. We respect your work.

###  **Template Superpowers**
```bash
forge-tree forge structure.txt --var project_name=MyApp --var author="Your Name"
```
Variables in your templates get replaced automagically. It's like mail merge, but for code.


##  Command Cheat Sheet

| What You Want | Command |
|---------------|---------|
| **Basic forging** | `forge-tree forge structure.txt` |
| **Custom location** | `forge-tree forge structure.txt -o ~/Projects` |
| **See what's happening** | `forge-tree forge structure.txt --verbose` |
| **YOLO mode (overwrite everything)** | `forge-tree forge structure.txt --force` |
| **Check if structure is valid** | `forge-tree validate structure.txt` |
| **Use variables** | `forge-tree forge structure.txt --var name=John --var email=john@example.com` |

## Pro Tips

### 1. **The "I'm Feeling Lucky" Approach**
Just describe your ideal project structure in a text file. Forge-Tree is pretty good at figuring out what you mean.

### 2. **The "Template Master" Move**
Create template files for your common project types. React app? Got a template. Rust CLI? Got a template. Microservice? You know it.

### 3. **The "Team Player" Strategy**
Share your structure files with your team. Now everyone can spin up identical project structures. No more "it works on my machine because I forgot to mention that obscure folder structure."

### 4. **The "Documentation Hero" Hack**
Include structure files in your project documentation. New team members can literally copy-paste and get started instantly.

##  When Things Go Wrong

**"It's not creating files!"**
- Check if you have write permissions
- Make sure file extensions are present (`.js`, `.py`, `.rs`)

**"It's creating everything as folders!"**
- Add file extensions or use the `--verbose` flag to see what's being detected

**"It says file already exists!"**
- Use `--force` if you want to overwrite, or change your output directory

**"My structure looks weird!"**
- Run `forge-tree validate structure.txt` first to catch formatting issues

##  What's Coming Next

- ** Web UI**: Because sometimes you want to click and drag
- ** Template Library**: Community-shared project templates
- ** Git Integration**: `forge-tree init --template rust-cli --git`
- ** IDE Extensions**: Right-click in VS Code, select "Forge Structure"
- ** AI Suggestions**: "I want to build a React app with authentication" → instant structure

##  Join the Forge Revolution

Found a bug? Have an idea? Want to contribute? We're all ears!

- ** Issues**: [GitHub Issues](https://github.com/IDKSAM27/forge-tree/issues)
- ** Ideas**: [Discussions](https://github.com/IDKSAM27/forge-tree/discussions)
- ** Contribute**: Fork, code, PR. You know the drill.

##  The Legal Stuff

MIT License - aka "do whatever you want with this, just don't sue me." 

Full license text is in the `LICENSE` file, but honestly, it's pretty standard stuff.

---

## The Bottom Line

**Forge-Tree** is what happens when a developer gets tired of manually creating project structures and decides to do something about it. It's fast, it's beautiful, it's useful, and it'll save you hours of mundane folder-clicking.

*Go forth and forge amazing projects!* 

---

**Made with 🦀 Rust, ❤️ Love, and Frustration at manual folder creation**

*P.S. - It took me hours to `forge` this README, you better have read it all or else get ready to feel my breath in your ears while you're sleeping. And Yes, I know there are other scaffolding tools. But have you seen how pretty my progress bars are?*
