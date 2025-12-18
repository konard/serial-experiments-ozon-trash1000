using SWEeM.Application.Dtos.Project;
using SWEeM.Domain.Entities;

namespace SWEeM.Application.Mappers;

public static class ProjectMappers
{
    public static Project ToProject(this CreateProjectDto dto)
        => new()
        {
            Id = Guid.NewGuid(),
            ClientId = dto.ClientId,
            Name = dto.Name,
            StartDate = dto.StartDate,
            PlannedEndDate = dto.PlannedEndDate,
            ActualEndDate = dto.ActualEndDate,
            ManagerId = dto.ManagerId
        };

    public static void UpdateFrom(this Project project, UpdateProjectDto dto)
    {
        project.Name = dto.Name;
        project.PlannedEndDate = dto.PlannedEndDate;
        project.ActualEndDate = dto.ActualEndDate;
        project.ManagerId = dto.ManagerId;
    }

    public static ProjectDto? ToDto(this Project? project)
    {
        if (project is null)
        {
            return null;
        }

        return new ProjectDto
        (
            project.Id,
            project.ClientId,
            project.Name,
            project.StartDate,
            project.PlannedEndDate,
            project.ActualEndDate,
            project.ManagerId
        );
    }
}