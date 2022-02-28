# Add support for attachments

## Description

When creating bug tickets, we usually want to attach crash dumps, screen shots, logs… We probably want to use Git LFS because we surely don't want local mirrors to automatically download, or even contain *all* attachments there ever was for a given project.

## Meta

| Field            | Value                                                        |
| ---------------- | ------------------------------------------------------------ |
| [Status](F001)   | [Open](ST001)                                                |
| [Type](F002)     | [Task](T001)                                                 |
| [Assignee](F003) | [Florian Simon](U001)                                        |
| [Required](F004) | &check;                                                      |
| Planned          | &cross;                                                      |
| Notes            | It's still a bit early to tackle this, we might want to wait |
|                  | a bit longer.                                                |

## Comments

### [Florian Simon](U001)
Although we shouldn't be overthinking this, I think it might be worth making choices that will > scale reasonably well in the long run.

### [Florian Simon](U001)
Also, it's not a top-priority. There are more important things to do first, such as parsing an issue file, for instance.
