
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <getopt.h>
#include "liquid.h"

#define INPUT_BUF_SIZE 8192
#define NUM_SAMPLES INPUT_BUF_SIZE/4 // float complex IQ pairs

typedef struct {
	int arg_samplerate;
	int samplerate;

	int arg_resamplerate;
	int resamplerate;

	int arg_bandwidth;
	int bandwidth;

	int arg_modulation;
	int modulation;

	int arg_fm_deviation;
	int fm_deviation;

} args_t;

void print_help() {
	fprintf(stderr, "demod takes signed 16 bit IQ data stream filter_attenuation input and produces signed 16 bit demodulated audio filter_attenuation output\n");
	fprintf(stderr, "usage: demod args\n");
	fprintf(stderr, "\t--samplerate \t-s <samplerate>\t\t: input data stream samplerate\n");
	fprintf(stderr, "\t--resamplerate \t-r <resamplerate>\t: output data stream samplerate\n");
	fprintf(stderr, "\t--bandwidth \t-b <bandwidth_hz>\t: input signal bandwidth\n\n");

	fprintf(stderr, "\t--modulation \t-m <fm>\t\t\t: input signal modulation\n");
	fprintf(stderr, "\t\t\t   <fm> -d=<deviation_hz>\n\n");

	fprintf(stderr, "\t--help \t\t-h \t\t\t: prints this usage information\n");
}

int main(int argc, char*argv[]) {
	// command line options
	int opt = 0;
	int long_index = 0;
	char* subopts;
	char* value;

	// modulation choices
	enum {
		FM_MODULATION = 0,
		AM_MODULATION,
	};

	const char* modulation_opts[] = {
		[FM_MODULATION] = "FM",
		[AM_MODULATION] = "AM",
		NULL
	};

	// FM modulation parameters
	enum {
		FM_MOD_DEVIATION = 0,
	};

	const char* fm_modulation_opts[] = {
		[FM_MOD_DEVIATION] = "d",
		NULL
	};

	args_t args;
	memset((void*)&args, 0, sizeof(args_t));
	static struct option long_options[] = {
		{"samplerate",		required_argument,	0,	's' }, // samplerate of input IQ data stream
		{"resamplerate",	required_argument,	0,	'r' }, // samplerate of output IQ data stream
		{"bandwidth",		required_argument,	0,	'b' }, // signal/lowpfilter_attenuations filter bandwidth

		{"modulation",		required_argument,	0,	'm' }, // which modulation is used

		{"help",			required_argument,	0,	'h' },
		{NULL,				0,				NULL,	 0	}
	};

	fprintf(stderr, "demod\t(C) 2015 Andres Vahter (andres.vahter@gmail.com)\n\n");

	while ((opt = getopt_long(argc, argv,"s:r:b:m:h", long_options, &long_index )) != -1) {
		switch (opt) {
			case 's' :
				args.arg_samplerate = 1;
				args.samplerate = atoi(optarg);
				if (args.samplerate <= 0) {
					fprintf(stderr, "samplerate must be > 0\n");
					exit(EXIT_FAILURE);
				}
				break;

			case 'r' :
				args.arg_resamplerate = 1;
				args.resamplerate = atoi(optarg);
				if (args.resamplerate <= 0) {
					fprintf(stderr, "resamplerate must be > 0\n");
					exit(EXIT_FAILURE);
				}
				break;

			case 'b' :
				args.arg_bandwidth = 1;
				args.bandwidth = atoi(optarg);
				if (args.bandwidth <= 0) {
					fprintf(stderr, "bandwidth must be > 0 Hz\n");
					exit(EXIT_FAILURE);
				}
				break;

			case 'm' :
				if (strcmp(modulation_opts[FM_MODULATION], optarg) == 0) {
					optarg = optarg + strlen(optarg) + 1; // consume "FM" argument
					subopts = optarg;

					while (*subopts != '\0') {
						char* saved = subopts;
						switch (getsubopt(&subopts, (char **)fm_modulation_opts, &value)) {
							case FM_MOD_DEVIATION:
								args.arg_fm_deviation = 1;
								args.fm_deviation = strtod(value, NULL);
								break;
							default:
								fprintf(stderr, "incorrect suboption: '%s'\n", saved);
								fprintf(stderr, "use FM demodulation filter_attenuation: -m FM d=<deviation_in_hz>\n");
								exit(EXIT_FAILURE);
						}
					}

					if (!args.arg_fm_deviation) {
						fprintf(stderr, "deviation eg. d=3500 is not specified for FM demodulation\n");
						exit(EXIT_FAILURE);
					}

					args.arg_modulation = 1;
					args.modulation = FM_MODULATION;
				}
				else {
					fprintf(stderr, "invalid modulation: %s\n", optarg);
					fprintf(stderr, "valid modulations are: <FM>\n"); // make it automatic if more demodulation options are added
					exit(EXIT_FAILURE);
				}

				break;

			case 'h' :
				print_help();
				exit(EXIT_SUCCESS);
				break;
			 default:
				print_help();
				exit(EXIT_FAILURE);
		}
	}


	// check args
	if (!args.arg_samplerate) {
		fprintf(stderr, "-s [--samplerate] not specified!\n");
		exit(EXIT_FAILURE);
	}
	else {
		fprintf(stderr, "input samplerate : %u\n", args.samplerate);
		if (!args.resamplerate) {
			args.resamplerate = args.samplerate;
		}
	}

	if (!args.bandwidth) {
		fprintf(stderr, "-b [--bandwidth] not specified!\n");
		exit(EXIT_FAILURE);
	}

	fprintf(stderr, "output samplerate: %u\n", args.resamplerate);
	fprintf(stderr, "signal bandwidth : %u Hz\n\n", args.bandwidth);

	fprintf(stderr, "using %s demodulation\n", modulation_opts[FM_MODULATION]);
	if (args.modulation == FM_MODULATION) {
		fprintf(stderr, "\tdeviation: ±%u Hz\n", args.fm_deviation);
	}


	unsigned int i;
	unsigned int k;
	uint8_t iq_buffer[INPUT_BUF_SIZE];

	// filter options
	unsigned int filter_len = 64;
	float filter_cutoff_freq = (float)args.bandwidth / (float)args.samplerate;
	float filter_attenuation = 70.0f; // stop-band attenuation
	// design filter from prototype and scale to bandwidth
	firfilt_crcf filter = firfilt_crcf_create_kaiser(filter_len, filter_cutoff_freq, filter_attenuation, 0.0f);
	firfilt_crcf_set_scale(filter, 2.0f*filter_cutoff_freq);

	// resampler options
    float resampler_rate = (float)args.resamplerate / (float)args.samplerate;
	msresamp_crcf resampler = msresamp_crcf_create(resampler_rate, filter_attenuation);
	float resampler_delay = msresamp_crcf_get_delay(resampler);

    // number of input samples (zero-padded)
    unsigned int resampler_input_len = NUM_SAMPLES + (int)ceilf(resampler_delay) + 10;
    // output buffer with extra padding
    unsigned int resampler_output_len = (unsigned int) (2*(float)resampler_input_len * resampler_rate);

    float complex c_input[resampler_input_len];
    float complex c_output[resampler_output_len];
	unsigned int resampler_output_count = 0;

    // FM demodulator
	float kf = (float)args.fm_deviation/(float)args.resamplerate;    // modulation factor
	freqdem fm_demodulator = freqdem_create(kf);
	float dem_out[resampler_output_len];

	while (1) {

		int bytes_read = fread(iq_buffer, 1, INPUT_BUF_SIZE, stdin);
		if (bytes_read) {

			// convert int16_t IQ to complex float
			for (k=0, i=0; k<bytes_read/2; k+=2, i++) {
				c_input[i] = ((int16_t*)&iq_buffer)[k] / 32768.0 + ((int16_t*)&iq_buffer)[k+1] / 32768.0 * I;

				// run filter
				firfilt_crcf_push(filter, c_input[i]);
				firfilt_crcf_execute(filter, &c_input[i]);
			}

			// resample
			msresamp_crcf_execute(resampler, c_input, NUM_SAMPLES, c_output, &resampler_output_count);

			// demodulate
			freqdem_demodulate_block(fm_demodulator, c_output, resampler_output_count, dem_out);

			if (0) {
				//float output
				fwrite((uint8_t*)dem_out, 1, resampler_output_count*4, stdout);
			}
			else {
				for (i=0; i<resampler_output_count; i++) {
					if (dem_out[i] > 1.0) {
						dem_out[i] = 1.0;
					}
					if (dem_out[i] < -1.0) {
						dem_out[i] = -1.0;
					}

					((int16_t*)&dem_out)[i] = dem_out[i] * 32767.0;

				}

				// i16 output
				fwrite((uint8_t*)dem_out, 1, resampler_output_count*2, stdout);
			}

			fflush(stdout);
		}

		if (feof(stdin)) {
			break;
		}
	}

	// destroy filter object
	firfilt_crcf_destroy(filter);

	// destroy resampler object
	msresamp_crcf_destroy(resampler);

	// destroy fm demodulator object
	freqdem_destroy(fm_demodulator);

	return 0;
}
