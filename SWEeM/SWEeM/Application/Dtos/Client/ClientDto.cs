namespace SWEeM.Application.Dtos.Client;

public record ClientDto(
    Guid Id,
    string Name,
    string Address,
    uint ProjectsTotal,
    uint ProjectsCompleted);