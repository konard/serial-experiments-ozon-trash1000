namespace SWEeM.Application.Dtos.Project;

public record ProjectDto(
    Guid Id,
    Guid ClientId,
    string Name,
    DateOnly StartDate,
    DateOnly PlannedEndDate,
    DateOnly ActualEndDate,
    Guid ManagerId);