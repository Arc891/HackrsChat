A quick doc to think about some stuff regarding design choices.

## Server db user structs

Something I've been thinking about is if there should exist multiple sub-structs of the User (db-entry) struct, like UserLogin, UserProfile etc. with partial fields of the User struct. This could be a clear indication of which information is being used or needed in a specific case, but it could also be a bit overkill or even unnecessary maybe? I'm not sure how to think about it.

My questions: 
- Are these "sub-structs" needed at all? -> User structs are all server-side in the end
- If using them: which sub-structs with which fields do we need? 