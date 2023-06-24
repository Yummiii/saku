using Discord.Interactions;

namespace Saku.Modules;

public class TestModule : InteractionModuleBase
{
    [SlashCommand("pitas", "manda mensagem fofa")]
    public async Task SendPitasGay()
    {
        await RespondAsync("fofa");
    }
}