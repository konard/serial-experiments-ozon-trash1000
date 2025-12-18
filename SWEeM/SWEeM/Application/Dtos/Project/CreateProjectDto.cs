namespace SWEeM.Application.Dtos.Project;

public record CreateProjectDto(
    Guid ClientId,
    string Name,
    DateOnly StartDate,
    DateOnly PlannedEndDate,
    DateOnly ActualEndDate,
    Guid ManagerId);