
---

# 3️⃣ PROJECT_STRUCTURE.md

`PROJECT_STRUCTURE.md`

```md
# Project Structure

server-attack-defense/

attack-tools/
    multi-stresser/
        src/
        Cargo.toml

defense-lab/
    analyzer/
        src/
        Cargo.toml

target/
.gitignore
README.md
LICENSE
SECURITY.md
CONTRIBUTING.md

---

## attack-tools

Simulates network stress and attack scenarios.

Current tools:
- multi-stresser

Planned:
- port scanner
- slowloris simulator

---

## defense-lab

Detects and analyzes suspicious activity.

Current modules:
- analyzer

Features:
- connection tracking
- attack detection
- JSON logging

---

## Future modules

- firewall integration
- intrusion detection
- real-time monitoring
