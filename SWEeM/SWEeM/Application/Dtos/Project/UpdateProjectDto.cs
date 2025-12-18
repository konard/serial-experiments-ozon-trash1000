namespace SWEeM.Application.Dtos.Project;

public record UpdateProjectDto(
    string Name,
    DateOnly PlannedEndDate,
    DateOnly ActualEndDate,
    Guid ManagerId);