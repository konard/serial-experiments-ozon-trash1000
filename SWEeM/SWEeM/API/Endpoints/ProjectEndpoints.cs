using SWEeM.Application.Dtos;
using SWEeM.Application.Dtos.Project;
using SWEeM.Application.Services;

namespace SWEeM.API.Endpoints;

public static class ProjectEndpoints
{
    public static RouteGroupBuilder MapProjectEndpoints(this WebApplication app)
    {
        var group = app.MapGroup("/projects")
            .WithTags("Projects")
            .WithDescription("Endpoints for managing software development projects");

        group.MapGet("/", async (
            ProjectService service,
            CancellationToken cancellationToken,
            int pageSize = 10,
            int page = 1) =>
        {
            var paginatedResult = await service.GetAllAsync(page, pageSize, cancellationToken);
            return Results.Ok(paginatedResult);
        })
        .WithName("GetAllProjects")
        .WithSummary("Get all projects with pagination")
        .WithDescription("Retrieves a paginated list of all projects with their details")
        .Produces<PaginatedResult<ProjectDto>>(200, "application/json")
        .ProducesProblem(500);

        group.MapGet("/{id:guid}", async (Guid id, ProjectService service, CancellationToken cancellationToken) =>
        {
            if (await service.GetByIdAsync(id, cancellationToken) is not { } project)
            {
                return Results.NotFound();
            }

            return Results.Ok(project);
        })
        .WithName("GetProjectById")
        .WithSummary("Get a project by ID")
        .WithDescription("Retrieves a specific project by its unique identifier")
        .Produces<ProjectDto>(200, "application/json")
        .Produces(404)
        .ProducesProblem(500);

        group.MapPost("/", async (CreateProjectDto dto, ProjectService service, CancellationToken cancellationToken) =>
        {
            try
            {
                var id = await service.CreateAsync(dto, cancellationToken);
                return TypedResults.Created($"/projects/{id}", id);
            }
            catch (ArgumentException ex)
            {
                return Results.BadRequest(ex.Message);
            }
        })
        .WithName("CreateProject")
        .WithSummary("Create a new project")
        .WithDescription("Creates a new project with the provided details including client, dates, and manager assignment")
        .Produces<Guid>(201, "application/json")
        .Produces(400)
        .ProducesProblem(500);

        group.MapPut("/{id:guid}", async (Guid id, UpdateProjectDto dto, ProjectService service, CancellationToken cancellationToken) =>
        {
            try
            {
                var project = await service.UpdateAsync(id, dto, cancellationToken);

                if (project is null)
                {
                    return Results.NotFound();
                }

                return Results.Ok(project);
            }
            catch (ArgumentException ex)
            {
                return Results.BadRequest(ex.Message);
            }
        })
        .WithName("UpdateProject")
        .WithSummary("Update an existing project")
        .WithDescription("Updates an existing project's information by its ID")
        .Produces<ProjectDto>(200, "application/json")
        .Produces(404)
        .Produces(400)
        .ProducesProblem(500);

        group.MapDelete("/{id:guid}", async (Guid id, ProjectService service, CancellationToken cancellationToken) =>
        {
            if (!await service.DeleteAsync(id, cancellationToken))
            {
                return Results.NotFound();
            }

            return Results.Ok(id);
        })
        .WithName("DeleteProject")
        .WithSummary("Delete a project")
        .WithDescription("Permanently deletes a project from the system by its ID")
        .Produces<Guid>(200, "application/json")
        .Produces(404)
        .ProducesProblem(500);

        return group;
    }
}
