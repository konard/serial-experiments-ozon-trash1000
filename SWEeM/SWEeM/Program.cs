using Microsoft.AspNetCore.Identity;
using Microsoft.EntityFrameworkCore;
using SWEeM.API.Endpoints;
using SWEeM.Application.Services;
using SWEeM.Domain.Entities;
using SWEeM.Infrastructure.Persistence;

var builder = WebApplication.CreateBuilder(args);

builder.Services.AddSwaggerGen(options =>
{
    options.SwaggerDoc("v1", new()
    {
        Title = "Swee API",
        Version = "v1",
        Description = "A minimal API for software development process management"
    });
});

builder.Services.AddDbContext<AppDbContext>(options =>
    options.UseSqlite("Data Source=swee.db"));

builder.Services.AddScoped<ClientService>();
builder.Services.AddScoped<ProjectService>();
builder.Services.AddScoped<UserService>();

builder.Services.AddScoped<IPasswordHasher<User>, PasswordHasher<User>>();

builder.Services.AddEndpointsApiExplorer();

var app = builder.Build();

if (app.Environment.IsDevelopment())
{
    app.UseSwagger();
    app.UseSwaggerUI();
}

app.UseHttpsRedirection();

app.MapClientEndpoints();
app.MapProjectEndpoints();
app.MapUserEndpoints();

app.Run();