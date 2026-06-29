// get open cases. This should be paginated, and the pagination offset is determined by the last received case ID from the previous request.
// Eg. GET /cases?after=OTHER_CASE_ULID&count=50
// It should also have a flag to filter by only the users cases & collaborated cases
// EG. GET /cases?owned=true&collaborated=true
// There should also be a flag to disallow sensitive cases, which should default to allowed if not specified.
// users without the sensitive permission should still have sensitive cases returned, but the evidence ids should be removed unless they are the owner/collaborator (assignable by admins/moderation leads)
