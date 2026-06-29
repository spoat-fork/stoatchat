// unhandled reports that the user is able to view.
// This route should be paginaged , and the pagination offset is determined by the last received case ID from the previous request.
// Eg. GET /reports?after=OTHER_REPORT_ULID&count=50
// There should be a way to specify whether oldest or newest reports are returned first
// Eg. GET /reports?first=new | GET /reports?first=old
// You should be able to filter by sensitive cases, with the default being combined (sensitive reports and not are both returned) (when permitted by the users permissions, otherwise false)
// Eg. GET /reports?sensitive=only | GET /reports?sensitive=none | GET /reports?sensitive=combined
