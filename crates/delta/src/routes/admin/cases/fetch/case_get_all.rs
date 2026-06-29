// This should return all cases (paginated similar to case_get_open) with the following filters:
// ```js
// {
//     state: OPEN | CLOSED,
//     owner: ULID,
//     collaborator: ULID, // cases this user collaborated on
//     before: Timestamp,
//     after: Timestamp,
//     tags: [tags],
//     sensitive: bool // cases that involve children (eg cp, underage, etc),
//     ... whatever else you can think of.
// }
// ```
// specifying owners/collaborators should be limited to admins/moderation leads
// the sensitive tag should be limited to some sort of "sensitive topics" permission
// users without the sensitive permission should still have sensitive cases returned, but the evidence ids should be removed.
