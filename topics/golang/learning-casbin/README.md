# Learning [Casbin](https://casbin.org/)

Based on https://zupzup.org/casbin-http-role-auth/

Tweaked for latest versions of library at time of writing

## Notes

* Extremely powerful
* Can integrate with DB's fairly easily for runtime management
* Absolutely perfect for tiered or granular access control
* Overkill for 90% of app scenarios
    * Maybe another 5% have an "admin" dashboard, where you just need an admin override to auth control, which is super simple in code
    * Last 5% with either multi-tenant setup with multi-level roles (eg: Buildkite, LaunchDarkly, etc.), it would be worth it
* Docs are pretty good, authors appear to be Chinese as some 70% of the tutorials are in Chinese, and eg: Java tutorials are 100% Chinese
    * Luckily murica loves its NodeJS
* Golang support is first class
    * Node and Java appear second class, rest are third and below
* Does support dynamic conditions (eg: temp tokens)
    * Its pretty clunky tho
    * [Requires manually adding the functions to each applicable group declaration](https://casbin.org/docs/rbac-with-conditions)
* Attribute based is supported
    * But it gets [spooky complex in some cases](https://casbin.org/docs/abac#scaling-the-model-for-complex-and-large-numbers-of-abac-rules)

## When to use

* Backend systems with >2 users in play
    * Eg: backend system serving users with multiple access levels (eg: family plan with lower-access kids in it) and CustSupport agents
    * Or something like AirBnB, with regular users, controllers of stays, group bookings, etc. Complex auth constraints
* Tiered/Group based auth
    * Eg: University, where faculty students and faculty staff can access faculty stuff, where students or staff are potentially across multiple faculties
* Data based authorisation
    * Eg: multi-tenant systems with multiple clients
