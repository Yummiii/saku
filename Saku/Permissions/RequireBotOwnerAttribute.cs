using Discord;
using Discord.Interactions;
using Microsoft.Extensions.DependencyInjection;
using Saku.ViewModels.Interfaces;

namespace Saku.Permissions;

[AttributeUsage(AttributeTargets.Class | AttributeTargets.Method)]
public class RequireBotOwnerAttribute : PreconditionAttribute
{
    public override Task<PreconditionResult> CheckRequirementsAsync(IInteractionContext context,
        ICommandInfo commandInfo, IServiceProvider services)
    {
        var config = services.GetRequiredService<ISakuConfig>();

        if (!ulong.TryParse(config.BotOwner, out var ownerDiscordId))
            return Task.FromResult(PreconditionResult.FromError("BotOwner is not valid ulong"));

        return Task.FromResult(context.User.Id == ownerDiscordId
            ? PreconditionResult.FromSuccess()
            : PreconditionResult.FromError("User is not bot owner"));
    }
}