# Driftwood
A simple desktop app that comes with a few things.
- API access to Netlify via OAuth2.0 for securly deploying a website.
- Functional and attractive templates for said website.
- A feature complete WYSIWYG text editor for writing blog posts.
- A friendly and fast interface for updating simple settings like website name, domain, etc.
- Templates are all SEO optimized so your blogs get the traffic they deserve.

## The goal
The easiest blog building experience, ever. 
It's like signing up for blogspot or medium. Pretty easy to create an account and start writing, right? Well picture that, except the blog is entirely your own. 
You own it, and it's content, and there are no watermarks, trademarks, other brands or companies logos. Just you.
The only account you'll need is via social sign-on through a serverless hosting provider (currently only Netlify is supported). Overall should be a very pain-free experience.

## Current state
At the moment, Driftwood's API is about 60% complete. This handles all the communicating with Netlify and the client's file system.

Still to be done:
- Secure OAuth2.0 web server for routing secure authentication requests (WIP)
- The entire GUI (Not yet started)
- Remaining Netlify API calls (things like updating site names, setting a domain, etc. has not yet been included).

Current Netlify API integration status:
- Create a site
- Deploy the site
- Propogate an SSL certificate (for some reason, it will likely never be used)

## Why
Driftwood is basically just a little hobby project I'm buidling to familiarize myself with Rust better. 
I hope it becomes a useful tool for anyone wanting to create a blog, but not one that relies on other platforms- it's entirely independant. 
If you really want, take the files Driftwood compiles and host them anywhere you want!

## Contribution
Due to how early on in development it is, Driftwood will remain a solo-project. Once it's off the ground, I'll be more than happy to welcome new templates, new features, and bug fixes.
One major feature will be additional serverless host providers, such as Vercel, Render, Cloudflare Pages, Github Pages, Railway, and whoever else has a public API. I didn't even check those, to be honest. Would be cool if we had more options though.
