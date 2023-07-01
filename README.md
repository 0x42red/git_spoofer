# Git Spoofer
Git spoofer is a simple tool that discovers the email address of an exist github user.
With this email address you can make it look like that person has committed to your repo.

# Why
There are a handful of people in the security space who view this as a pretty big issue.
A malicious user can use typo squatting mixed with this unverified attack to trick people into running malicious code.

This would be a significantly smaller problem if Github didn't take unverified commits and add them to your contributors list.
This combined with linking to official profiles under the github log allows for a convincing ruse.
![List of spoofed commits](https://github.com/0x42red/git_spoofer/blob/main/commits.png?raw=true)
![List of spoofed contributors](https://github.com/0x42red/git_spoofer/blob/main/contrib.png?raw=true)

# What about verified commits
Yes you can PGP sign your commits.  Which is great but all it does is give you a green badge.  This is an anti-pattern, similar to when HTTP was grey and HTTPS was green.
Github knows when I commit to a repo with no contributors that Linus Torvold's account was not related to my project. 

# Usage
Git spoofer simply amends your most recent commit so the usage is very simple.
```
# make a normal git commit then run the git_spoofer command
git_spoofer 0x42red
```

# Note
This was a solo project ran by just 0x42RED, Linus, Jakoby, Primeagen, etc did not help me write this.  But you would have no real way of knowing that.
