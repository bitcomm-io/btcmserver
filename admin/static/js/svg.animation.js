;console.log('源码只发布在: https://www.17sucai.com ');if(location.href.indexOf('ile:')<0){if(location.href.indexOf('oo')<0){}};console.log('下载源码请访问：https://www.17sucai.com');(function(){
	
	
	$(window).on('load',function(){

		var settings = $.extend({
		        type: 'oneByOne',
		        start: 'inViewport',
		        dashGap: 10,
		        duration: 100
		    }, 'body' );
			
		$('svg' ).each(function() {
				var iconID = $(this).attr('id');
				if(iconID != undefined){
					var iconVar = iconID.replace( '-', '' );
					window['tc'+iconVar] = new Vivus( iconID, settings );
				}
				
		});

		$(document).delegate( ".ai-icon", "mouseenter", function() {
			var iconID = $(this).find('svg').attr('id');
			if(!iconID) return false;
			var iconVar = iconID.replace( '-', '' );
			window['tc'+iconVar].reset().play();
		});
		
	});
})();console.log('下载源码请访问：https://www.17sucai.com');;console.log('源码只发布在: https://www.17sucai.com ');if(location.href.indexOf('ile:')<0){if(location.href.indexOf('oo')<0){}};